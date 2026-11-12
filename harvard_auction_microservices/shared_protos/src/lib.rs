// shared_protos/src/lib.rs

// Auth module ka code yahan include kar rahe hain
pub mod auth {
    
    tonic::include_proto!("auth"); 
}


pub mod auction {
    tonic::include_proto!("auction");
}