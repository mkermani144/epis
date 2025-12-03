//! Epis router

pub mod handlers;

use axum::{Router, routing::any};

use crate::{
  domain::ports::{Epis, UserManagement},
  inbound::{http::AppStateV2, ws::epis::handlers::chat::chat},
};

/// Epis subrouter
#[derive(Debug, Clone)]
pub struct EpisWsRouter<E: Epis, UM: UserManagement>(Router<AppStateV2<E, UM>>);

impl<E: Epis, UM: UserManagement> EpisWsRouter<E, UM> {
  /// Construct Epis router
  pub fn new() -> Self {
    let router = Router::new().route("/chat/{chatmate_id}", any(chat));

    Self(router)
  }

  /// Convert to inner router
  pub fn into_inner(self) -> Router<AppStateV2<E, UM>> {
    self.0
  }
}
