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
  providers::llm::Llm, rag::rag::Rag,
};

/// HTTP server manager using Axum
pub struct HttpServer {
  router: Router,
  addr: SocketAddr,
}

#[derive(Clone)]
pub struct AppState<L: Llm, CR: ConversationRepository, R: Rag> {
  pub lingoo: Arc<Lingoo<L, CR, R>>,
  pub conversation_repository: Arc<CR>,
  pub rag: Arc<R>,
}

#[derive(Clone)]
pub struct ConversationAppState<CR: ConversationRepository> {
  pub conversation_repository: Arc<CR>,
}

#[derive(Clone)]
pub struct LingooAppState<L: Llm, CR: ConversationRepository, R: Rag> {
  pub lingoo: Arc<Lingoo<L, CR, R>>,
  // FIXME: Remove this field when /lingoo/conversation/list API is fixed
  pub conversation_repository: Arc<CR>,
  pub rag: Arc<R>,
}

#[derive(OpenApi)]
struct ApiDoc;

impl HttpServer {
  /// Creates a new HTTP server
  pub fn try_new<L: Llm, CR: ConversationRepository, R: Rag>(
    addr: SocketAddr,
    app_state: AppState<L, CR, R>,
  ) -> Result<Self> {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
      .nest("/conversation", ConversationRouter::new().into_inner())
      .with_state(ConversationAppState {
        conversation_repository: app_state.conversation_repository.clone(),
      })
      .nest("/lingoo", LingooRouter::new().into_inner())
      .with_state(LingooAppState {
        lingoo: app_state.lingoo.clone(),
        conversation_repository: app_state.conversation_repository.clone(),
        rag: app_state.rag.clone(),
      })
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
