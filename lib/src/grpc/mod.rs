pub mod internal {
    tonic::include_proto!("cpc.internal");
}

pub mod client;
pub mod server;
pub mod error;