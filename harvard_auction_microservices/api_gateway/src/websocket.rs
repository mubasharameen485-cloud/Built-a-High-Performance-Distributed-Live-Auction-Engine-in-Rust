// api_gateway/src/websocket.rs

use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::Response,
};
use serde::{Deserialize, Serialize};
use tracing::{info, error};
use crate::AppState;

use shared_protos::auction::auction_service_client::AuctionServiceClient;
use shared_protos::auction::{PlaceBidRequest as GrpcBidReq, SaveChatRequest as GrpcChatReq};

// Frontend se aane wali aur jane wali JSON ka format
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WsMessage {
    pub msg_type: String, 
    pub item_id: Option<i32>, // Option banaya taake 'null' par crash na ho
    pub user_id: Option<i32>,
    pub sender: Option<String>,
    pub amount: Option<f64>,
    pub content: Option<String>,
}

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let mut rx = state.tx.subscribe();
    info!("Naya user Live Arena mein dakhil hua!");

    loop {
        tokio::select! {
            // ==========================================
            // 1. FRONTEND SE MESSAGE AAYA
            // ==========================================
            msg_result = socket.recv() => {
                let msg = match msg_result {
                    Some(Ok(m)) => m,
                    Some(Err(e)) => { error!("Socket error: {}", e); break; }
                    None => { info!("User chala gaya."); break; }
                };

                if let Message::Text(text) = msg {
                    info!("Frontend se RAW text aaya: {}", text);

                    if let Ok(ws_data) = serde_json::from_str::<WsMessage>(&text) {
                        
                        // item_id nikal lo, agar null hai toh 0 rakh do
                        let safe_item_id = ws_data.item_id.unwrap_or(0);

                        if let Ok(mut client) = AuctionServiceClient::connect("http://[::1]:50052").await {
                            
                            // ------------------------------------------
                            // AGAR BOLI (BID) HAI
                            // ------------------------------------------
                            if ws_data.msg_type == "bid" {
                                info!("Gaya gRPC ke paas Boli lagane: ${}", ws_data.amount.unwrap_or(0.0));
                                
                                let request = tonic::Request::new(GrpcBidReq {
                                    item_id: safe_item_id,
                                    user_id: ws_data.user_id.unwrap_or(0),
                                    bid_amount: ws_data.amount.unwrap_or(0.0),
                                });

                                match client.place_bid(request).await {
                                    Ok(response) => {
                                        let inner = response.into_inner();
                                        if inner.success {
                                            info!("gRPC ne Boli manzoor kar li! Nayi qeemat: ${}", inner.new_price);
                                            
                                            // Broadcast the new bid
                                            let broadcast_msg = WsMessage {
                                                msg_type: "bid_update".to_string(),
                                                item_id: Some(safe_item_id), // Yahan theek kiya
                                                user_id: ws_data.user_id,
                                                sender: None,
                                                amount: Some(inner.new_price),
                                                content: Some(inner.message),
                                            };
                                            
                                            let json_str = serde_json::to_string(&broadcast_msg).unwrap();
                                            let _ = state.tx.send(json_str);
                                        }
                                    }
                                    Err(e) => {
                                        error!("gRPC ne Boli reject kar di: {}", e.message());
                                    }
                                }
                            } 
                            // ------------------------------------------
                            // AGAR CHAT HAI
                            // ------------------------------------------
                            else if ws_data.msg_type == "chat" {
                                info!("Chat message aaya: {}", ws_data.content.clone().unwrap_or_default());
                                
                                let request = tonic::Request::new(GrpcChatReq {
                                    item_id: safe_item_id,
                                    sender: ws_data.sender.clone().unwrap_or_default(),
                                    content: ws_data.content.clone().unwrap_or_default(),
                                });

                                let _ = client.save_chat(request).await;

                                let broadcast_msg = WsMessage {
                                    msg_type: "chat_update".to_string(),
                                    item_id: Some(safe_item_id), // Yahan theek kiya
                                    user_id: None,
                                    sender: ws_data.sender,
                                    amount: None,
                                    content: ws_data.content,
                                };
                                let _ = state.tx.send(serde_json::to_string(&broadcast_msg).unwrap());
                            }
                        } else {
                            error!("gRPC Auction Service se connect nahi ho saka!");
                        }
                    } else {
                        error!("Frontend se aane wala JSON format ghalat hai!");
                    }
                }
            }

            // ==========================================
            // 2. CHANNEL SE BROADCAST AAYA (Sab ko bhejna hai)
            // ==========================================
            channel_result = rx.recv() => {
                if let Ok(broadcast_msg_str) = channel_result {
                    if let Err(e) = socket.send(Message::Text(broadcast_msg_str)).await {
                        error!("Frontend ko message bhejne mein masla: {}", e);
                        break;
                    }
                }
            }
        }
    }
}