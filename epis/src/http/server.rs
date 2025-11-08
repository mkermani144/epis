use anyhow::Result;
use axum::Router;
use clerk_rs::{
  clerk::Clerk,
  validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
};
use epis_stt::stt::Stt;
use epis_tts::tts::Tts;
use std::{
  net::SocketAddr,
  sync::{Arc, Mutex},
};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

use crate::{
  ai::{llm::Llm, router::AiRouter},
  conversation::{repository::ConversationRepository, router::ConversationRouter},
  lingoo::{
    lingoo::Lingoo,
    router::{LingooRouter, LingooWebsocketRouter},
  },
  rag::rag::Rag,
};

#[derive(Debug)]
pub struct HttpServer {
  router: Router,
  addr: SocketAddr,
}

// This newtype is needed only for implementing Debug
#[derive(Clone)]
pub struct ClerkWrapper(Clerk);
impl ClerkWrapper {
  pub fn new(clerk: Clerk) -> Self {
    Self(clerk)
  }
  pub fn into_inner(self) -> Clerk {
    self.0
  }
}
impl std::fmt::Debug for ClerkWrapper {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Clerk").finish()
  }
}

#[derive(Debug)]
pub struct AppState<L: Llm, CR: ConversationRepository, R: Rag, S: Stt, T: Tts> {
  pub lingoo: Arc<Lingoo<L, CR, R>>,
  pub conversation_repository: Arc<CR>,
  pub llm: Arc<L>,
  pub stt: Arc<Mutex<S>>,
  pub tts: Arc<Mutex<T>>,
  pub clerk: ClerkWrapper,
}
// Stt is not Clone for now, so we need to impl Clone
impl<L: Llm, CR: ConversationRepository, R: Rag, S: Stt, T: Tts> Clone
  for AppState<L, CR, R, S, T>
{
  fn clone(&self) -> Self {
    Self {
      lingoo: self.lingoo.clone(),
      conversation_repository: self.conversation_repository.clone(),
      llm: self.llm.clone(),
      stt: self.stt.clone(),
      tts: self.tts.clone(),
      clerk: self.clerk.clone(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct ConversationAppState<CR: ConversationRepository> {
  pub conversation_repository: Arc<CR>,
}

// TODO: Extract WS state so it's not part of REST Lingoo state
#[derive(Debug)]
pub struct LingooAppState<L: Llm, CR: ConversationRepository, R: Rag, S: Stt, T: Tts> {
  pub lingoo: Arc<Lingoo<L, CR, R>>,
  // FIXME: Remove this field when /lingoo/conversation/list API is fixed
  pub conversation_repository: Arc<CR>,
  pub stt: Arc<Mutex<S>>,
  pub tts: Arc<Mutex<T>>,
  pub clerk: ClerkWrapper,
}
// Stt is not Clone for now, so we need to impl Clone
impl<L: Llm, CR: ConversationRepository, R: Rag, S: Stt, T: Tts> Clone
  for LingooAppState<L, CR, R, S, T>
{
  fn clone(&self) -> Self {
    Self {
      lingoo: self.lingoo.clone(),
      conversation_repository: self.conversation_repository.clone(),
      stt: self.stt.clone(),
      tts: self.tts.clone(),
      clerk: self.clerk.clone(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct AiAppState<L: Llm> {
  pub llm: Arc<L>,
}

#[derive(OpenApi)]
struct ApiDoc;

impl HttpServer {
  /// Creates a new HTTP server
  pub fn try_new<L: Llm, CR: ConversationRepository, R: Rag, S: Stt, T: Tts>(
    addr: SocketAddr,
    app_state: AppState<L, CR, R, S, T>,
    // TODO: Switch to a solution-agnostic trait
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
        stt: app_state.stt.clone(),
        tts: app_state.tts.clone(),
        clerk: app_state.clerk.clone(),
      })
      .nest("/ai", AiRouter::new().into_inner())
      .with_state(AiAppState {
        llm: app_state.llm.clone(),
      })
      .split_for_parts();

    // TODO: Add a root WS router and put the logic there
    // TODO: Document the WS router somehow
    let router = router
      .nest("/ws/lingoo", LingooWebsocketRouter::new().into_inner())
      .with_state(LingooAppState {
        lingoo: app_state.lingoo.clone(),
        conversation_repository: app_state.conversation_repository.clone(),
        stt: app_state.stt.clone(),
        tts: app_state.tts.clone(),
        clerk: app_state.clerk.clone(),
      });

    // Layers that apply to both REST and WS
    let router = router
      .layer(ClerkLayer::new(
        MemoryCacheJwksProvider::new(app_state.clerk.clone().into_inner()),
        None,
        true,
      ))
      .layer(TraceLayer::new_for_http());
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
