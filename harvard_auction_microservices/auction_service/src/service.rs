// auction_service/src/service.rs

use tonic::{Request, Response, Status};
use sqlx::PgPool;
use tracing::{info, error};

use shared_protos::auction::auction_service_server::AuctionService;
use shared_protos::auction::*;

pub struct MyAuctionService {
    pub pool: PgPool,
}

#[tonic::async_trait]
impl AuctionService for MyAuctionService {
    
    // 1. Create Item (Admin)
    async fn create_item(&self, request: Request<CreateItemRequest>) -> Result<Response<ItemResponse>, Status> {
        let req = request.into_inner();
        let result = sqlx::query!(
            "INSERT INTO auction_items (name, description, current_price, image_url) VALUES ($1, $2, $3, $4) RETURNING id",
            req.name, req.description, req.starting_price, req.image_url
        )
        .fetch_one(&self.pool).await.map_err(|_| Status::internal("DB Error"))?;

        Ok(Response::new(ItemResponse { item_id: result.id, message: "Item Created".to_string() }))
    }

    // 2. Place Bid (User)
    async fn place_bid(&self, request: Request<PlaceBidRequest>) -> Result<Response<BidResponse>, Status> {
        let req = request.into_inner();
        let mut tx = self.pool.begin().await.map_err(|_| Status::internal("Tx Error"))?;

        let item = sqlx::query!("SELECT current_price FROM auction_items WHERE id = $1 FOR UPDATE", req.item_id)
            .fetch_optional(&mut *tx).await.map_err(|_| Status::internal("DB Error"))?;

        let current_price = match item {
            Some(record) => record.current_price,
            None => return Err(Status::not_found("Item not found")),
        };

        if req.bid_amount <= current_price {
            return Err(Status::failed_precondition("Bid must be higher than current price"));
        }

        let _ = sqlx::query!("UPDATE auction_items SET current_price = $1 WHERE id = $2", req.bid_amount, req.item_id)
            .execute(&mut *tx).await;
        let _ = sqlx::query!("INSERT INTO bids (item_id, user_id, bid_amount) VALUES ($1, $2, $3)", req.item_id, req.user_id, req.bid_amount)
            .execute(&mut *tx).await;

        tx.commit().await.map_err(|_| Status::internal("Commit Error"))?;

        Ok(Response::new(BidResponse { success: true, message: "Bid Placed".to_string(), new_price: req.bid_amount }))
    }

    // 3. Get Active Item (Page Refresh par sab se naya item lana)
    async fn get_active_item(&self, _req: Request<Empty>) -> Result<Response<ActiveItemResponse>, Status> {
        let item = sqlx::query!(
            "SELECT id, name, description, current_price, image_url FROM auction_items ORDER BY created_at DESC LIMIT 1"
        )
        .fetch_optional(&self.pool).await.map_err(|_| Status::internal("DB Error"))?;

        match item {
            Some(record) => Ok(Response::new(ActiveItemResponse {
                item_id: record.id,
                name: record.name,
                description: record.description,
                current_price: record.current_price,
                image_url: record.image_url.unwrap_or_default(),
            })),
            None => Err(Status::not_found("No active items")),
        }
    }

    // 4. Get Bids History
    async fn get_item_bids(&self, request: Request<ItemRequest>) -> Result<Response<BidsListResponse>, Status> {
        let req = request.into_inner();
        let records = sqlx::query!("SELECT user_id, bid_amount, created_at::TEXT as time FROM bids WHERE item_id = $1 ORDER BY created_at DESC LIMIT 20", req.item_id)
            .fetch_all(&self.pool).await.unwrap_or_default();

        let bids = records.into_iter().map(|r| BidRecord {
            user_id: r.user_id, amount: r.bid_amount, created_at: r.time.unwrap_or_default()
        }).collect();

        Ok(Response::new(BidsListResponse { bids }))
    }

    // 5. Get Chats History
    async fn get_item_chats(&self, request: Request<ItemRequest>) -> Result<Response<ChatsListResponse>, Status> {
        let req = request.into_inner();
        let records = sqlx::query!("SELECT sender, content, created_at::TEXT as time FROM live_chats WHERE item_id = $1 ORDER BY created_at ASC LIMIT 50", req.item_id)
            .fetch_all(&self.pool).await.unwrap_or_default();

        let chats = records.into_iter().map(|r| ChatMessage {
            sender: r.sender, content: r.content, created_at: r.time.unwrap_or_default()
        }).collect();

        Ok(Response::new(ChatsListResponse { chats }))
    }

    // 6. Save Live Chat
    async fn save_chat(&self, request: Request<SaveChatRequest>) -> Result<Response<Empty>, Status> {
        let req = request.into_inner();
        let _ = sqlx::query!("INSERT INTO live_chats (item_id, sender, content) VALUES ($1, $2, $3)", req.item_id, req.sender, req.content)
            .execute(&self.pool).await;
        Ok(Response::new(Empty {}))
    }
}