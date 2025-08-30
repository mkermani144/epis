use anyhow::Result;
use axum::Router;
use log::info;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

use crate::{
  conversation::{repository::ConversationRepository, router::ConversationRouter},
  lingoo::{lingoo::Lingoo, router::LingooRouter},
  providers::llm::Llm,
};

/// HTTP server manager using Axum
pub struct HttpServer {
  router: Router,
  addr: SocketAddr,
}

#[derive(Clone)]
pub struct AppState<T: Llm, R: ConversationRepository> {
  pub lingoo: Arc<Lingoo<T, R>>,
  pub conversation_repository: Arc<R>,
}

#[derive(OpenApi)]
struct ApiDoc;

impl HttpServer {
  /// Creates a new HTTP server
  pub fn try_new<T: Llm, R: ConversationRepository>(
    addr: SocketAddr,
    app_state: AppState<T, R>,
  ) -> Result<Self> {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
      .nest("/conversation", ConversationRouter::new().into_inner())
      .nest("/lingoo", LingooRouter::new().into_inner())
      .with_state(app_state)
      .split_for_parts();

    let router = router.merge(Scalar::with_url("/scalar", api));
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
