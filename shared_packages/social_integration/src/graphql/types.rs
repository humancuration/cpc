//! GraphQL types for social integration features

use async_graphql::{SimpleObject, InputObject, Enum, ID};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};