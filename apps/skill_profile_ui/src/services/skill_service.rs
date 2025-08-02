use tonic_web_wasm_client::Client;
use wasm_bindgen::prelude::*;
use crate::grpc::skill_volunteering::{
    skill_volunteering_service_client::SkillVolunteeringServiceClient, AddUserSkillRequest,
    ListSkillsRequest, ListUserSkillsRequest, RemoveUserSkillRequest, UserSkillDetails,
};

// This should be configured somewhere, but for now, we'll hardcode it.
const GRPC_ENDPOINT: &str = "http://127.0.0.1:50051";

#[wasm_bindgen]
pub struct SkillService {
    client: SkillVolunteeringServiceClient<Client>,
}

#[wasm_bindgen]
impl SkillService {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let client = SkillVolunteeringServiceClient::new(Client::new(GRPC_ENDPOINT.to_string()));
        Self { client }
    }

    pub async fn list_user_skills(&mut self, user_id: String) -> Result<JsValue, JsValue> {
        let request = tonic::Request::new(ListUserSkillsRequest { user_id });
        let response = self.client.list_user_skills(request).await.map_err(|e| JsValue::from_str(&e.to_string()))?;
        let skills = response.into_inner().user_skills;
        Ok(serde_wasm_bindgen::to_value(&skills).unwrap())
    }

    pub async fn remove_user_skill(&mut self, user_id: String, skill_id: String) -> Result<JsValue, JsValue> {
        let request = tonic::Request::new(RemoveUserSkillRequest { user_id, skill_id });
        let response = self.client.remove_user_skill(request).await.map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(serde_wasm_bindgen::to_value(&response.into_inner().success).unwrap())
    }

    pub async fn add_user_skill(
        &mut self,
        user_id: String,
        skill_id: String,
        skill_level: String,
    ) -> Result<JsValue, JsValue> {
        let request = tonic::Request::new(AddUserSkillRequest {
            user_id,
            skill_id,
            skill_level,
        });
        let response = self
            .client
            .add_user_skill(request)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(serde_wasm_bindgen::to_value(&response.into_inner().user_skill).unwrap())
    }

    pub async fn list_skills(
        &mut self,
        limit: i32,
        offset: i32,
        category: Option<String>,
    ) -> Result<JsValue, JsValue> {
        let request = tonic::Request::new(ListSkillsRequest {
            category,
            limit: Some(limit),
            offset: Some(offset),
        });
        let response = self
            .client
            .list_skills(request)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let skills = response.into_inner().skills;
        Ok(serde_wasm_bindgen::to_value(&skills).unwrap())
    }
}

// We need to implement Default so we can use it in the Yew component state.
impl Default for SkillService {
    fn default() -> Self {
        Self::new()
    }
}