//! Websocket auth middleware

use std::collections::HashMap;

use axum::{
  extract::{Query, State},
  http::StatusCode,
  middleware::Next,
  response::IntoResponse,
};

use crate::{
  domain::{
    models::AuthStatus,
    ports::{Epis, UserManagement},
  },
  inbound::http::AppState,
};

/// A middleware to authenticate routes, adding user extension to request
pub async fn auth_layer<E: Epis, UM: UserManagement>(
  State(state): State<AppState<E, UM>>,
  query: Query<HashMap<String, String>>,
  mut request: axum::extract::Request,
  next: Next,
) -> axum::response::Response {
  let token = if request.uri().path().contains("ws") {
    query.get("jwt").cloned()
  } else {
    request
      .headers()
      .get("Authorization")
      .and_then(|header| header.to_str().ok())
      .map(|full_token| full_token.replace("Bearer ", ""))
  };
  if let Some(jwt) = token {
    if let Ok(auth_status) = state.user_management().authenticate_jwt(&jwt).await {
      return match auth_status {
        AuthStatus::Authenticated(user) => {
          request.extensions_mut().insert(user);
          next.run(request).await
        }
        AuthStatus::Unauthenticated => {
          (StatusCode::UNAUTHORIZED, "Unauthorized jwt").into_response()
        }
      };
    }

    (StatusCode::INTERNAL_SERVER_ERROR, "Cannot complete auth").into_response()
  } else {
    (StatusCode::BAD_REQUEST, "jwt not provided").into_response()
  }
}
