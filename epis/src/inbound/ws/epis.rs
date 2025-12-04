//! Epis router

pub mod handlers;

use axum::{Router, routing::any};

use crate::{
  domain::ports::{Epis, UserManagement},
  inbound::{http::AppState, ws::epis::handlers::chat::chat},
};

/// Epis subrouter
#[derive(Debug, Clone)]
pub struct EpisWsRouter<E: Epis, UM: UserManagement>(Router<AppState<E, UM>>);

impl<E: Epis, UM: UserManagement> EpisWsRouter<E, UM> {
  /// Construct Epis router
  pub fn new() -> Self {
    let router = Router::new().route("/chat/{chatmate_id}", any(chat));

    Self(router)
  }

  /// Convert to inner router
  pub fn into_inner(self) -> Router<AppState<E, UM>> {
    self.0
  }
}
