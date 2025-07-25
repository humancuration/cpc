use tauri::{State, Window};
use cpc_core::supply_chain::{service::SupplyChainService, models::SupplyChain};

#[derive(serde::Serialize, Clone)]
pub struct CommandError {
    message: String,
}

impl<T: std::error::Error> From<T> for CommandError {
    fn from(err: T) -> Self {
        CommandError { message: err.to_string() }
    }
}

#[tauri::command]
pub async fn get_supply_chain(
    product_id: String,
    service: State<'_, SupplyChainService>
) -> Result<SupplyChain, CommandError> {
    service.get_supply_chain(&product_id).await.map_err(Into::into)
}

#[tauri::command]
pub async fn subscribe_to_supply_chain_updates(
    window: Window,
    product_id: String,
    service: State<'_, SupplyChainService>
) -> Result<(), CommandError> {
    let mut rx = service.get_update_stream();
    tokio::spawn(async move {
        while let Ok(update) = rx.recv().await {
            if update.product_id == product_id {
                let _ = window.emit("supply-chain-update", Some(update.clone()));
            }
        }
    });
    Ok(())
}