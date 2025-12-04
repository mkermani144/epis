//! Http server for Epis

use anyhow::Result;
use axum::{
  Router,
  http::{self, HeaderValue},
  middleware::from_fn_with_state,
};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

use crate::{
  domain::ports::{Epis, UserManagement},
  inbound::{auth_layer::auth_layer, rest::epis::EpisRouter, ws::epis::EpisWsRouter},
};

/// Epis HTTP server
#[derive(Debug)]
#[allow(clippy::missing_docs_in_private_items)]
pub struct HttpServer {
  router: Router,
  addr: SocketAddr,
}

/// AppState used inside HTTP layer
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Debug, Clone, derive_getters::Getters)]
pub struct AppState<E: Epis, UM: UserManagement> {
  epis: Arc<E>,
  user_management: Arc<UM>,
}

#[derive(OpenApi)]
#[allow(clippy::missing_docs_in_private_items)]
struct ApiDoc;

impl HttpServer {
  /// Creates a new HTTP server
  pub fn try_new<E: Epis, UM: UserManagement>(
    addr: SocketAddr,
    app_url: &str,
    epis: Arc<E>,
    user_management: Arc<UM>,
  ) -> Result<Self> {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
      .nest("/v2/epis", EpisRouter::new().into_inner())
      .with_state(AppState {
        epis: epis.clone(),
        user_management: user_management.clone(),
      })
      .split_for_parts();

    let router = router
      .nest("/v2/epis/ws", EpisWsRouter::new().into_inner())
      .with_state(AppState {
        epis: epis.clone(),
        user_management: user_management.clone(),
      });

    // Layers that apply to both REST and WS
    let mut router = router
      .layer(from_fn_with_state(
        AppState {
          epis: epis.clone(),
          user_management: user_management.clone(),
        },
        auth_layer,
      ))
      .layer(TraceLayer::new_for_http())
      .layer(
        CorsLayer::new()
          .allow_origin(app_url.parse::<HeaderValue>()?)
          .allow_credentials(true)
          .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION]),
      );

    router = router.merge(Scalar::with_url("/scalar", api));

    info!("HTTP server initialized successfully");

    Ok(Self { router, addr })
  }

  /// Starts the HTTP server and listens for incoming connections
  pub async fn start(self) -> Result<()> {
    let listener = TcpListener::bind(self.addr).await?;
    info!("HTTP server listening on {}", self.addr);

    axum::serve(listener, self.router).await?;
    Ok(())
  }
}
