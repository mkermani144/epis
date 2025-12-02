//! Epis router

pub mod handlers;

use axum::{Router, routing::any};

use crate::{
  domain::ports::Epis,
  inbound::{http::AppStateV2, ws::epis::handlers::chat::chat},
};

/// Epis subrouter
#[derive(Debug, Clone)]
pub struct EpisWsRouter<E: Epis>(Router<AppStateV2<E>>);

impl<E: Epis> EpisWsRouter<E> {
  /// Construct Epis router
  pub fn new() -> Self {
    let router = Router::new().route("/chat/{chatmate_id}", any(chat));

    Self(router)
  }

  /// Convert to inner router
  pub fn into_inner(self) -> Router<AppStateV2<E>> {
    self.0
  }
}
