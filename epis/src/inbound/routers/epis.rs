//! Epis router

pub mod handlers;

use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  domain::ports::Epis,
  inbound::{
    http::AppStateV2,
    routers::epis::handlers::handshake_chatmate::{__path_handshake_chatmate, handshake_chatmate},
  },
};

/// Category used in Scalar
pub const EPIS_CATEGORY: &str = "Epis";

/// Epis subrouter
#[derive(Debug, Clone)]
pub struct EpisRouter<E: Epis>(OpenApiRouter<AppStateV2<E>>);

impl<E: Epis> EpisRouter<E> {
  /// Construct Epis router
  pub fn new() -> Self {
    let router = OpenApiRouter::new().routes(routes!(handshake_chatmate));

    Self(router)
  }

  /// Convert to inner router
  pub fn into_inner(self) -> OpenApiRouter<AppStateV2<E>> {
    self.0
  }
}
