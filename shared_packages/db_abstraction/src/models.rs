//! Domain models mapped to database tables
//!
//! This module contains the domain models that are mapped to database tables
//! using Diesel's derive macros.

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

/// Example user model
/// In a real application, this would be specific to the application domain
#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// New user model for insert operations
#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
}

/// Update user model for update operations
#[derive(AsChangeset, Debug, Clone)]
#[diesel(table_name = crate::schema::users)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
}

/// Example entity model
/// This demonstrates how to create models for different entities
#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::entities)]
pub struct Entity {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// New entity model for insert operations
#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = crate::schema::entities)]
pub struct NewEntity {
    pub name: String,
    pub description: Option<String>,
}