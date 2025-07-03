use axum::extract::Request;
use axum::http;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;

use common::auth::jwt::{Claims, decode_token};
use models::dto::sys::response::sys_user::ResponseUser;

pub async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());
    let auth_header = if let Some(data) = auth_header {
        data
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let user = authorize_current_user::<>(auth_header).await?;
    req.extensions_mut().insert(user.user);
    Ok(next.run(req).await)
}

async fn authorize_current_user(auth_token: &str) -> Result<Claims<ResponseUser>, StatusCode>
where
{
    let user = decode_token(auth_token);
    if let Ok(data) = user {
        Ok(data)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}