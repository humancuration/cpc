use serde::{Serialize, Deserialize};
use crate::file_utils::ChunkDiff;

#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkMessage {
    ChangeNotification(ChangeNotification),
    ChangeAck(ChangeAck),
    // New messages for file synchronization
    FileChangeNotification(FileChangeNotification),
    FilePatchRequest(FilePatchRequest),
    FilePatch(FilePatch),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeNotification {
    pub file_id: String,
    pub version: u64,
    pub diff: Vec<ChunkDiff>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeAck {
    pub file_id: String,
    pub accepted: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileChangeNotification {
    pub file_path: String,
    pub merkle_root: String,
    pub version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilePatchRequest {
    pub file_path: String,
    pub base_version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilePatch {
    pub file_path: String,
    pub patches: Vec<u8>, // Serialized patch data
    pub new_version: u64,
}