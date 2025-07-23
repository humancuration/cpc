use async_graphql::{Context, Object, Result};
use ed25519_dalek::PublicKey;
use crate::AppState;

pub struct PublicKeyQuery;

#[Object]
impl PublicKeyQuery {
    async fn impact_public_key(&self, ctx: &Context<'_>) -> Result<String> {
        let state = ctx.data::<Arc<AppState>>()?;
        Ok(base64::encode(state.verification_key.as_bytes()))
    }
}