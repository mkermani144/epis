use std::path::Path;

use derive_more::Display;
use epis_core::non_empty_text::NonEmptyString;
use rten::{Model, ModelLoadError};
use rten_generate::Generator;
use rten_tensor::{AsView, Layout, NdTensor};
use thiserror::Error;

use crate::{models::TtsLanguage, ttp::Ttp};

pub struct ByT5Ttp {
  encoder: Model,
  decoder: Model,
}

impl std::fmt::Debug for ByT5Ttp {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ByT5Ttp")
      .field("encoder", &"<Rten Encoder>")
      .field("decoder", &"<Rten Decoder>")
      .finish()
  }
}

#[derive(Debug, Clone, Display)]
pub enum ByT5ModelType {
  Encoder,
  Decoder,
}

#[derive(Debug, Error)]
pub enum ByT5TtpInitError {
  #[error("{0} model not found in the provided path")]
  NotFoundModel(ByT5ModelType),
  #[error("Unknown error while loading {0} model")]
  Unknown(ByT5ModelType),
}

impl ByT5Ttp {
  /// Initialize a [ByT5Ttp]
  /// To get models, follow these commands which gives you a pair for .rten models:
  /// ```
  /// optimum-cli export onnx --model fdemelo/g2p-mbyt5-12l-ipa-childes-espeak g2p --task text2text-generation-with-past
  /// rten-convert g2p/encoder_model.onnx
  /// rten-convert g2p/decoder_model_merged.onnx
  /// ```
  ///
  /// These commands are copied from:
  /// https://github.com/robertknight/rten/blob/4f2524673ebc09ad4243e2dc153e18b50dbac8c7/rten-examples/src/byt5_g2p.rs
  pub fn new(
    encoder_model_path: &Path,
    decoder_model_path: &Path,
  ) -> Result<Self, ByT5TtpInitError> {
    Ok(Self {
      encoder: Model::load_file(encoder_model_path).map_err(|e| match e {
        ModelLoadError::ReadFailed(_) => ByT5TtpInitError::NotFoundModel(ByT5ModelType::Encoder),
        _ => ByT5TtpInitError::Unknown(ByT5ModelType::Encoder),
      })?,
      decoder: Model::load_file(decoder_model_path).map_err(|e| match e {
        ModelLoadError::ReadFailed(_) => ByT5TtpInitError::NotFoundModel(ByT5ModelType::Decoder),
        _ => ByT5TtpInitError::Unknown(ByT5ModelType::Decoder),
      })?,
    })
  }
}

// NOTE: The following logic is totally copied from Rten examples
// https://github.com/robertknight/rten/blob/4f2524673ebc09ad4243e2dc153e18b50dbac8c7/rten-examples/src/byt5_g2p.rs

const BOS_ID: u32 = 0;
const EOS_ID: u32 = 1;
const SPECIAL_TOKEN_COUNT: u32 = 3;
const MAX_TOKENS: usize = 512;

fn encode_text(text: &str) -> Vec<i32> {
  text
    .as_bytes()
    .iter()
    .map(|c| *c as u32 + SPECIAL_TOKEN_COUNT)
    .map(|id| id as i32)
    .collect()
}
fn decode_ids(ids: &[u32]) -> String {
  let bytes: Vec<u8> = ids
    .iter()
    .filter(|x| **x >= SPECIAL_TOKEN_COUNT)
    .map(|x| (*x - SPECIAL_TOKEN_COUNT) as u8)
    .collect();
  String::from_utf8_lossy(&bytes).to_string()
}

#[derive(Debug, Error)]
pub enum ByT5TtpError {
  #[error("Internal error while converting text to phonemes using Rten")]
  RtenInternal,
  #[error("Empty Ttp output")]
  EmptyOutput,
}

impl Ttp for ByT5Ttp {
  type Error = ByT5TtpError;

  fn text_to_phonemes(
    &mut self,
    text: &NonEmptyString,
    language: &TtsLanguage,
  ) -> Result<NonEmptyString, Self::Error> {
    let prompt = format!("<{}>: {}", language.to_string(), text);
    let input_ids = encode_text(prompt.as_ref());
    let input_ids = NdTensor::from_data([1, input_ids.len()], input_ids);
    let attention_mask = NdTensor::full([1, input_ids.len()], 1);

    let [encoded_state] = self
      .encoder
      .run_n(
        [
          (
            self
              .encoder
              .node_id("input_ids")
              .map_err(|_| ByT5TtpError::RtenInternal)?,
            input_ids.into(),
          ),
          (
            self
              .encoder
              .node_id("attention_mask")
              .map_err(|_| ByT5TtpError::RtenInternal)?,
            attention_mask.view().into(),
          ),
        ]
        .into(),
        [self
          .encoder
          .node_id("last_hidden_state")
          .map_err(|_| ByT5TtpError::RtenInternal)?],
        None,
      )
      .map_err(|_| ByT5TtpError::RtenInternal)?;
    let encoded_state: NdTensor<f32, 3> = encoded_state
      .try_into()
      .map_err(|_| ByT5TtpError::RtenInternal)?;

    let generator = Generator::from_model(&self.decoder)
      .map_err(|_| ByT5TtpError::RtenInternal)?
      .with_constant_input(
        self
          .decoder
          .node_id("encoder_attention_mask")
          .map_err(|_| ByT5TtpError::RtenInternal)?,
        attention_mask.view().into(),
      )
      .with_constant_input(
        self
          .decoder
          .node_id("encoder_hidden_states")
          .map_err(|_| ByT5TtpError::RtenInternal)?,
        encoded_state.view().into(),
      )
      .with_prompt(&[BOS_ID])
      .take(MAX_TOKENS);

    let mut token_ids = Vec::new();
    for token_id in generator {
      let token = token_id.map_err(|_| ByT5TtpError::RtenInternal)?;
      if token == EOS_ID {
        break;
      }
      token_ids.push(token);
    }

    let phonemes: NonEmptyString = decode_ids(&token_ids)
      .try_into()
      .map_err(|_| ByT5TtpError::EmptyOutput)?;

    Ok(phonemes)
  }
}
