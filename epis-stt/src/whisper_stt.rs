use std::path::Path;

use thiserror::Error;
use whisper_rs::{
  DtwModelPreset, FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters,
  WhisperState,
};

use crate::{
  models::{AudioBytes, SttLanguage},
  stt::{Stt, SttError},
};

#[derive(Debug, Clone, Error)]
pub enum WhisperSttConstructionError {
  #[error("the provided path is not valid")]
  InvalidPath,
  #[error("unknown error during model context construction")]
  Unknown,
}

// FIXME: Check if original_rate is larger than 16kHz
fn resample_down_to_16khz(samples: &[f32], original_rate: u32) -> Vec<f32> {
  let target_rate = 16000f32;
  let ratio = original_rate as f32 / target_rate;
  let target_len = (samples.len() as f32 / ratio) as usize;

  (0..target_len)
    .map(|i| {
      let source_idx = (i as f32 * ratio) as usize;
      samples[source_idx]
    })
    .collect()
}

// TODO: Add other presets
#[derive(Debug, Clone)]
pub enum WhisperModelPreset {
  Tiny,
}
impl From<WhisperModelPreset> for DtwModelPreset {
  fn from(value: WhisperModelPreset) -> Self {
    match value {
      WhisperModelPreset::Tiny => DtwModelPreset::Tiny,
    }
  }
}

#[derive(Debug)]
pub struct WhisperStt {
  state: WhisperState,
}

impl WhisperStt {
  /// Construct a [`WhisperStt`] from a path and preset.
  ///
  /// # arguments
  /// - model_path: A path to a whisper model. The model can be downloaded from https://huggingface.co/ggerganov/whisper.cpp/tree/main
  /// - model_preset: The preset of the model downloaded
  pub fn try_new(
    model_path: &Path,
    model_preset: WhisperModelPreset,
  ) -> Result<Self, WhisperSttConstructionError> {
    let mut context_param = WhisperContextParameters::default();
    context_param.dtw_parameters.mode = whisper_rs::DtwMode::ModelPreset {
      model_preset: model_preset.into(),
    };

    let ctx = WhisperContext::new_with_params(
      &model_path
        .to_str()
        .ok_or(WhisperSttConstructionError::InvalidPath)?,
      context_param,
    )
    .map_err(|_| WhisperSttConstructionError::Unknown)?;

    let state = ctx
      .create_state()
      .map_err(|_| WhisperSttConstructionError::Unknown)?;

    Ok(Self { state })
  }

  pub fn init_whisper_params(language: &str) -> FullParams<'_, '_> {
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 5 });

    params.set_language(Some(language));
    params.set_n_threads(std::thread::available_parallelism().map_or(4, |p| p.get() as i32));
    params.set_print_progress(false);
    params.set_print_timestamps(false);

    params
  }
}

impl Stt for WhisperStt {
  /// Convert speech in a specific language (via its bytes) to a text consumable.
  /// The bytes provided should be in wav format.
  fn speech_to_text(
    &mut self,
    bytes: &AudioBytes,
    language: SttLanguage,
  ) -> Result<impl IntoIterator<Item = &str>, SttError> {
    let reader = hound::WavReader::new(bytes.as_slice()).map_err(|_| SttError::InvalidBytes)?;

    let hound::WavSpec {
      channels,
      sample_rate,
      ..
    } = reader.spec();

    let samples = reader
      .into_samples::<i16>()
      .collect::<Result<Vec<i16>, _>>()
      .map_err(|_| SttError::FailedSampling)?;

    let mut audio = vec![
      0.0f32;
      samples
        .len()
        .try_into()
        .map_err(|_| SttError::FailedSampling)?
    ];
    whisper_rs::convert_integer_to_float_audio(&samples, &mut audio)
      .map_err(|_| SttError::FailedSampling)?;

    if channels == 2 {
      let mut temp_audio = vec![0.0f32; audio.len() / 2];
      whisper_rs::convert_stereo_to_mono_audio(&audio, &mut temp_audio)
        .map_err(|_| SttError::FailedSampling)?;
      audio = temp_audio;
    } else if channels != 1 {
      return Err(SttError::UnsupportedSorroundAudio);
    }

    if sample_rate != 16000 {
      audio = resample_down_to_16khz(&audio, sample_rate)
    }

    let params = Self::init_whisper_params(language.as_ref());

    self
      .state
      .full(params, &audio[..])
      .map_err(|_| SttError::ModelError)?;

    // NOTE: I'm not sure if falling back to an empty string makes sense here
    Ok(self.state.as_iter().map(|s| s.to_str().unwrap_or("")))
  }
}
