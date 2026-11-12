// shared_protos/src/lib.rs

// Auth module ka code yahan include kar rahe hain
pub mod auth {
    
    tonic::include_proto!("auth"); 
}

// Auction module ka code yahan include kar rahe hain
pub mod auction {
    tonic::include_proto!("auction");
}