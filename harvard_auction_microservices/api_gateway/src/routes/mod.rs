// api_gateway/src/routes/mod.rs

pub mod auth;
pub mod auction;

use axum::Router;
use crate::AppState;

// Dono files ke routes ko mila kar ek Master Router banate hain
pub fn create_router() -> Router<AppState> {
    Router::new()
        .merge(auth::auth_routes())
        .merge(auction::auction_routes())
}