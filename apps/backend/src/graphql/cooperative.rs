use async_graphql::{Context, InputObject, Object, Result, ID};
use cpc_core::cooperative::{
    models::{Cooperative, CooperativeMember},
    repository::{AddMemberData, CreateCooperativeData, UpdateCooperativeData},
};
use crate::{
    cooperative::service::{CooperativeService, ServiceError},
    graphql::identity::UserType,
    core::services::user::UserService,
    utils::graphql::AppContext,
};
use std::sync::Arc;
use uuid::Uuid;
use chrono::NaiveDate;

#[Object]
impl Cooperative {
    async fn id(&self) -> Uuid {
        self.id
    }
    async fn name(&self) -> &str {
        &self.name
    }
    async fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    async fn founded_date(&self) -> String {
        self.founded_date.to_string()
    }
    async fn website(&self) -> Option<&str> {
        self.website.as_deref()
    }
    async fn members(&self, ctx: &Context<'_>) -> Result<Vec<CooperativeMember>> {
        let service = ctx.data_unchecked::<Arc<CooperativeService>>();
        service.list_members(self.id).await.map_err(|e| e.to_string().into())
    }
}

#[Object]
impl CooperativeMember {
    async fn id(&self) -> Uuid {
        self.id
    }
    async fn cooperative_id(&self) -> Uuid {
        self.cooperative_id
    }
    async fn role(&self) -> &str {
        &self.role
    }
    async fn user(&self, ctx: &Context<'_>) -> Result<UserType> {
        let app_ctx = AppContext::from(ctx);
        let user = app_ctx.user_service.get_user_by_id(self.user_id).await?;
        Ok(user.into())
    }
    async fn joined_at(&self) -> String {
        self.joined_at.to_rfc3339()
    }
}

#[derive(Default)]
pub struct CooperativeQueryRoot;

#[Object]
impl CooperativeQueryRoot {
    async fn cooperatives(&self, ctx: &Context<'_>) -> Result<Vec<Cooperative>> {
        let service = ctx.data_unchecked::<Arc<CooperativeService>>();
        service.list_cooperatives().await.map_err(|e| e.to_string().into())
    }

    async fn cooperative(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Cooperative>> {
        let service = ctx.data_unchecked::<Arc<CooperativeService>>();
        match service.get_cooperative(id).await {
            Ok(cooperative) => Ok(Some(cooperative)),
            Err(ServiceError::NotFound) => Ok(None),
            Err(e) => Err(e.to_string().into()),
        }
    }

    async fn members(&self, ctx: &Context<'_>, cooperative_id: Uuid) -> Result<Vec<CooperativeMember>> {
        let service = ctx.data_unchecked::<Arc<CooperativeService>>();
        service.list_members(cooperative_id).await.map_err(|e| e.to_string().into())
    }
}

#[derive(InputObject)]
struct CreateCooperativeInput {
    name: String,
    description: Option<String>,
    founded_date: String,
    website: Option<String>,
}

#[derive(InputObject)]
struct UpdateCooperativeInput {
    id: Uuid,
    name: Option<String>,
    description: Option<String>,
    website: Option<String>,
}

#[derive(InputObject)]
struct AddMemberInput {
    cooperative_id: Uuid,
    user_id: Uuid,
    role: String,
}

#[derive(Default)]
pub struct CooperativeMutationRoot;

#[Object]
impl CooperativeMutationRoot {
    async fn create_cooperative(&self, ctx: &Context<'_>, input: CreateCooperativeInput) -> Result<Cooperative> {
        let service = ctx.data_unchecked::<Arc<CooperativeService>>();
        let founded_date = NaiveDate::parse_from_str(&input.founded_date, "%Y-%m-%d")
            .map_err(|_| "Invalid date format, expected YYYY-MM-DD")?;
        let data = CreateCooperativeData {
            name: input.name,
            description: input.description,
            founded_date,
            website: input.website,
        };
        service.create_cooperative(&data).await.map_err(|e| e.to_string().into())
    }

    async fn update_cooperative(&self, ctx: &Context<'_>, input: UpdateCooperativeInput) -> Result<Cooperative> {
        let service = ctx.data_unchecked::<Arc<CooperativeService>>();
        let data = UpdateCooperativeData {
            name: input.name,
            description: input.description,
            website: input.website,
        };
        service.update_cooperative(input.id, &data).await.map_err(|e| e.to_string().into())
    }

    async fn delete_cooperative(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let service = ctx.data_unchecked::<Arc<CooperativeService>>();
        service.delete_cooperative(id).await.map(|_| true).map_err(|e| e.to_string().into())
    }

    async fn add_member(&self, ctx: &Context<'_>, input: AddMemberInput) -> Result<CooperativeMember> {
        let service = ctx.data_unchecked::<Arc<CooperativeService>>();
        let data = AddMemberData {
            cooperative_id: input.cooperative_id,
            user_id: input.user_id,
            role: input.role,
        };
        service.add_member(&data).await.map_err(|e| e.to_string().into())
    }

    async fn remove_member(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let service = ctx.data_unchecked::<Arc<CooperativeService>>();
        service.remove_member(id).await.map(|_| true).map_err(|e| e.to_string().into())
    }

    async fn update_member_role(&self, ctx: &Context<'_>, member_id: Uuid, role: String) -> Result<CooperativeMember> {
        let service = ctx.data_unchecked::<Arc<CooperativeService>>();
        service.update_member_role(member_id, &role).await.map_err(|e| e.to_string().into())
    }
}