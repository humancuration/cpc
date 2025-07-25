pub mod graphql;
pub mod grpc;
pub mod service;

use crate::repositories::expense_repository::ExpenseRepositoryImpl;
use cpc_core::expenses::service::ExpenseService;
use service::ExpenseServiceImpl;
use sqlx::PgPool;
use std::sync::Arc;

pub fn create_service(pool: PgPool) -> Arc<dyn ExpenseService> {
    let expense_repo = Arc::new(ExpenseRepositoryImpl::new(pool));
    let expense_service = Arc::new(ExpenseServiceImpl::new(expense_repo));
    expense_service
}