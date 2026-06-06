// api_gateway/src/routes/auction.rs

use axum::{routing::{get, post}, Json, Router, http::StatusCode, extract::Path};
use serde::{Deserialize, Serialize};
use crate::AppState;

use shared_protos::auction::auction_service_client::AuctionServiceClient;
use shared_protos::auction::{CreateItemRequest as GrpcCreateItemReq, Empty, ItemRequest};

// Admin naya item banayega (Ab image_url bhi aayega)
#[derive(Deserialize)]
pub struct CreateItemPayload {
    pub name: String,
    pub description: String,
    pub starting_price: f64,
    pub image_url: Option<String>, 
}

#[derive(Serialize)]
pub struct ItemResponse {
    pub item_id: i32,
    pub message: String,
}

// History bhejne ke liye Structs
#[derive(Serialize)]
pub struct ActiveItemResponse {
    pub item_id: i32,
    pub name: String,
    pub description: String,
    pub current_price: f64,
    pub image_url: String,
}

#[derive(Serialize)]
pub struct HistoryResponse {
    pub bids: Vec<serde_json::Value>,
    pub chats: Vec<serde_json::Value>,
}

pub fn auction_routes() -> Router<AppState> {
    Router::new()
        .route("/items", post(create_item_handler)) // Admin Route
        .route("/items/active", get(get_active_item_handler)) // Page refresh par item lana
        .route("/items/:id/history", get(get_history_handler)) // Page refresh par purani chat aur bids lana
}

// 1. Admin Create Item
async fn create_item_handler(Json(payload): Json<CreateItemPayload>) -> Result<Json<ItemResponse>, (StatusCode, String)> {
    let mut client = AuctionServiceClient::connect("http://[::1]:50052").await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Auction Service Offline".to_string()))?;

    let request = tonic::Request::new(GrpcCreateItemReq {
        name: payload.name,
        description: payload.description,
        starting_price: payload.starting_price,
        image_url: payload.image_url.unwrap_or_default(),
    });

    let response = client.create_item(request).await.map_err(|e| (StatusCode::BAD_REQUEST, e.message().to_string()))?;
    let inner = response.into_inner();

    Ok(Json(ItemResponse { item_id: inner.item_id, message: inner.message }))
}

// 2. Get Active Item (Frontend load hote hi yeh call karega)
async fn get_active_item_handler() -> Result<Json<ActiveItemResponse>, (StatusCode, String)> {
    let mut client = AuctionServiceClient::connect("http://[::1]:50052").await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Service Offline".to_string()))?;
    
    let response = client.get_active_item(tonic::Request::new(Empty {})).await.map_err(|_| (StatusCode::NOT_FOUND, "No items found".to_string()))?;
    let inner = response.into_inner();

    Ok(Json(ActiveItemResponse {
        item_id: inner.item_id,
        name: inner.name,
        description: inner.description,
        current_price: inner.current_price,
        image_url: inner.image_url,
    }))
}

// 3. Get History (Bids aur Chats)
async fn get_history_handler(Path(item_id): Path<i32>) -> Result<Json<HistoryResponse>, (StatusCode, String)> {
    let mut client = AuctionServiceClient::connect("http://[::1]:50052").await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Service Offline".to_string()))?;
    
    let req = tonic::Request::new(ItemRequest { item_id });
    
    // Bids aur Chats dono gRPC se mangwao
    let bids_res = client.get_item_bids(req).await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Bids Error".to_string()))?;
    
    let req2 = tonic::Request::new(ItemRequest { item_id });
    let chats_res = client.get_item_chats(req2).await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Chats Error".to_string()))?;

    // gRPC ke data ko JSON mein badalna
    let bids = bids_res.into_inner().bids.into_iter().map(|b| serde_json::json!({ "user_id": b.user_id, "amount": b.amount, "time": b.created_at })).collect();
    let chats = chats_res.into_inner().chats.into_iter().map(|c| serde_json::json!({ "sender": c.sender, "content": c.content, "time": c.created_at })).collect();

    Ok(Json(HistoryResponse { bids, chats }))
}