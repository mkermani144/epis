use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  ai::{
    handlers::generate_title::{__path_generate_title, generate_title},
    llm::Llm,
  },
  http::server::AiAppState,
};

pub const AI_CATEGORY: &'static str = "Ai";

pub struct AiRouter<L: Llm>(OpenApiRouter<AiAppState<L>>);

impl<L: Llm> AiRouter<L> {
  pub fn new() -> Self {
    let router = OpenApiRouter::new().routes(routes!(generate_title));

    Self(router)
  }

  pub fn into_inner(self) -> OpenApiRouter<AiAppState<L>> {
    self.0
  }
}
