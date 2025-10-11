use std::{collections::HashMap, io::Cursor, path::Path};

use epis_core::non_empty_text::NonEmptyString;
use hound::{SampleFormat, WavSpec, WavWriter};
use rten::{Model, ModelLoadError, RunOptions};
use rten_tensor::{AsView, Layout, NdLayout, NdTensor, Tensor, TensorBase};
use thiserror::Error;

use crate::{
  kokoro::create_vocab::create_vocab,
  models::{AudioChunk, TtsLanguage},
  ttp::Ttp,
  tts::Tts,
};

pub struct KokoroTts<T: Ttp> {
  ttp: T,
  model: Model,
  voice_data: HashMap<TtsLanguage, TensorBase<Vec<f32>, NdLayout<2>>>,
}
impl<T: Ttp> std::fmt::Debug for KokoroTts<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("KokoroTts").finish()
  }
}

#[derive(Debug, Clone, Error)]
pub enum KokoroTtsInitError {
  #[error("Invalid voice data path: {0}")]
  InvalidVoiceDataPath(String),
  #[error("Invalid model path")]
  InvalidModelPath,
  #[error("Unknown error while loading Kokoro model")]
  Unknown,
}

// NOTE: All of the following Tts logic is copied from:
// https://github.com/robertknight/rten/blob/e723452593ed303a114ed415df2cc0587fd3091b/rten-examples/src/kokoro.rs

impl<T: Ttp> KokoroTts<T> {
  /// Initialize [KokoroTts]
  ///
  /// ## Arguments
  /// - ttp: A [Ttp] instance, used for getting phonemes of the text
  /// - model_path: Path to the Kokoro model path
  /// - supported_languages: A list of supported languages for Tts
  /// - voice_data_dir: Path to a directory cotaining all voice data for the supported languages, in
  /// the form of <lang_spec>.bin (e.g. "en-US.bin")
  ///
  /// ## Errors
  /// The funtion returns an error if:
  /// - The voice data file for any of the languages provided cannot be found
  /// - The model cannot be loaded for some reason
  ///
  /// ## Kokoro model
  /// Download the ONNX model from:
  /// https://huggingface.co/robertknight/kokoro-onnx/tree/main
  /// then convert it using rten-convert:
  /// ```
  /// rten-convert kokoro.onnx
  /// ```
  /// ## Voice data
  /// Download voice data from here and structure the files as mentioned above:
  /// https://huggingface.co/onnx-community/Kokoro-82M-v1.0-ONNX/tree/main/voices
  pub fn new(
    ttp: T,
    model_path: &Path,
    supported_languages: Vec<TtsLanguage>,
    voice_data_dir: &Path,
  ) -> Result<Self, KokoroTtsInitError> {
    let voice_data: HashMap<TtsLanguage, TensorBase<Vec<f32>, NdLayout<2>>> = supported_languages
      .into_iter()
      .map(
        |lang_spec| -> Result<(TtsLanguage, TensorBase<Vec<f32>, NdLayout<2>>), KokoroTtsInitError> {
          let voice_data_file = format!("{}/{}.bin", voice_data_dir.to_string_lossy(), lang_spec);
          let data: Vec<f32> = std::fs::read(&voice_data_file)
            .map_err(|_| KokoroTtsInitError::InvalidVoiceDataPath(voice_data_file))?
            .as_chunks()
            .0
            .iter()
            .copied()
            .map(f32::from_le_bytes)
            .collect();

          let style_dim = 256;
          let max_tokens = data.len() / style_dim;
          let voice_data = NdTensor::from_data(
            [max_tokens, style_dim],
            data,
          );

          Ok((lang_spec, voice_data))
        },
      )
      .collect::<Result<HashMap<TtsLanguage, TensorBase<Vec<f32>, NdLayout<2>>>, KokoroTtsInitError>>()?
      .into();

    let model = Model::load_file(model_path).map_err(|e| match e {
      ModelLoadError::ReadFailed(_) => KokoroTtsInitError::InvalidModelPath,
      _ => KokoroTtsInitError::Unknown,
    })?;

    Ok(Self {
      ttp,
      voice_data,
      model,
    })
  }
}

const BOS_ID: i32 = 0;

#[derive(Debug, Clone, Error)]
pub enum KokoroTtsError {
  #[error("Error during Ttp phase")]
  Ttp(String),
  #[error("Unsupported Ttp language: {0}")]
  UnsupportedLanguage(String),
  #[error("Internal Rten error during Ttp")]
  RtenInternal,
  #[error("Error while writing to wav stream")]
  WavWriter,
}

// TODO: Map to meaningful errors
impl<T: Ttp> Tts for KokoroTts<T> {
  type Error = KokoroTtsError;

  fn text_to_speech(
    &mut self,
    text: &NonEmptyString,
    language: &TtsLanguage,
  ) -> Result<impl IntoIterator<Item = AudioChunk>, Self::Error> {
    let phonemes = self
      .ttp
      .text_to_phonemes(text, &language)
      .map_err(|e| KokoroTtsError::Ttp(e.to_string()))?;

    let vocab = create_vocab();
    let mut input_ids = vec![BOS_ID];
    let mut ch_buf = vec![0u8; 8];
    let phonemes_str: &str = phonemes.as_ref();
    for ch in phonemes_str.chars() {
      let ch_str = ch.encode_utf8(&mut ch_buf);
      if let Some(token) = vocab.get(ch_str) {
        input_ids.push(*token as i32);
      }
    }
    input_ids.push(BOS_ID);

    let tokens = NdTensor::from_data([1, input_ids.len()], input_ids);

    let style_dim = 256;
    let voice_data = self
      .voice_data
      .get(&language)
      .ok_or(KokoroTtsError::UnsupportedLanguage(language.to_string()))?;
    let max_tokens = voice_data.len() / style_dim;
    let num_tokens = tokens.size(1).saturating_sub(2).min(max_tokens - 1);
    let style = voice_data.slice((num_tokens..num_tokens + 1, ..));

    let speed = NdTensor::from([1.]);

    let [output] = self
      .model
      .run_n(
        [
          (
            self
              .model
              .node_id("input_ids")
              .map_err(|_| KokoroTtsError::RtenInternal)?,
            tokens.into(),
          ),
          (
            self
              .model
              .node_id("style")
              .map_err(|_| KokoroTtsError::RtenInternal)?,
            style.into(),
          ),
          (
            self
              .model
              .node_id("speed")
              .map_err(|_| KokoroTtsError::RtenInternal)?,
            speed.into(),
          ),
        ]
        .into(),
        [self
          .model
          .node_id("waveform")
          .map_err(|_| KokoroTtsError::RtenInternal)?],
        Some(RunOptions {
          ..Default::default()
        }),
      )
      .map_err(|_| KokoroTtsError::RtenInternal)?;

    let audio: Tensor<f32> = output
      .try_into()
      .map_err(|_| KokoroTtsError::RtenInternal)?;

    let mut cursor = Cursor::new(Vec::new());
    let mut wav_writer = WavWriter::new(
      &mut cursor,
      WavSpec {
        channels: 1,
        sample_rate: 24_000,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
      },
    )
    .map_err(|_| KokoroTtsError::WavWriter)?;

    for sample in audio.iter() {
      wav_writer
        .write_sample(*sample)
        .map_err(|_| KokoroTtsError::WavWriter)?;
    }
    wav_writer
      .finalize()
      .map_err(|_| KokoroTtsError::WavWriter)?;

    Ok(vec![AudioChunk::new(cursor.into_inner())])
  }
}
