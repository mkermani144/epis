//! Epis router

pub mod handlers;

use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  domain::ports::{Epis, UserManagement},
  inbound::{
    http::AppState,
    rest::epis::handlers::{
      handshake_chatmate::{__path_handshake_chatmate, handshake_chatmate},
      list_chatmates::{__path_list_chatmates, list_chatmates},
    },
  },
};

/// Category used in Scalar
pub const EPIS_CATEGORY: &str = "Epis";

/// Epis subrouter
#[derive(Debug, Clone)]
pub struct EpisRouter<E: Epis, UM: UserManagement>(OpenApiRouter<AppState<E, UM>>);

impl<E: Epis, UM: UserManagement> EpisRouter<E, UM> {
  /// Construct Epis router
  pub fn new() -> Self {
    let router = OpenApiRouter::new().routes(routes!(handshake_chatmate, list_chatmates));

    Self(router)
  }

  /// Convert to inner router
  pub fn into_inner(self) -> OpenApiRouter<AppState<E, UM>> {
    self.0
  }
}
