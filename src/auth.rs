use crate::api_key;

use axum::{
    extract::FromRequestParts,
    extract::Request,
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use sqlx::types::Uuid;

pub(crate) struct AuthUser {
    pub(crate) key_id: Uuid,
    pub(crate) permission: String,
}

impl FromRequestParts<crate::DatabaseState> for AuthUser {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &crate::DatabaseState,
    ) -> Result<Self, Self::Rejection> {
        let Some(header_value) = parts.headers.get("bearer") else {
            return Err((StatusCode::UNAUTHORIZED, "Missing auth token"));
        };

        let api_key = header_value.to_str().map_err(|err| {
            tracing::warn!("Invalid authorization header: {:?}", err);
            (StatusCode::UNAUTHORIZED, "Invalid auth token")
        })?;

        let key_hash = api_key::hash_key(api_key);
        let result = sqlx::query!(
            "SELECT key_id, permission FROM 
            api_keys WHERE
            key_hash = $1",
            key_hash
        )
        .fetch_optional(&state.pool)
        .await
        .map_err(|err| {
            tracing::error!("Failed to execute query: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
        })?;

        let Some(auth) = result else {
            return Err((StatusCode::UNAUTHORIZED, "Invalid auth token"));
        };

        Ok(AuthUser {
            key_id: auth.key_id,
            permission: auth.permission,
        })
    }
}

pub(crate) async fn require_admin(
    auth_user: AuthUser,
    req: Request,
    next: Next,
) -> Response {
    if auth_user.permission != "admin" {
        return (StatusCode::FORBIDDEN, "insufficient priviledges")
            .into_response();
    }
    next.run(req).await
}
