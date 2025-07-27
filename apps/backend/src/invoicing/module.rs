//! Module initialization and wiring for the invoicing system

use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;
use cpc_net::net::Network;
use libp2p_core::PeerId;

use super::repositories::{InvoiceRepository, PgInvoiceRepository};
use super::graphql::{InvoicingQuery, InvoicingMutation, InvoicingSubscription};
use super::infrastructure::p2p::P2PInvoiceSharing;

/// This struct holds all the pieces the backend needs from this module
pub struct InvoicingModule {
    pub router: Router,
    pub query: InvoicingQuery,
    pub mutation: InvoicingMutation,
    pub subscription: InvoicingSubscription,
    pub p2p_sharing: Arc<P2PInvoiceSharing>,
}

/// This function initializes the module and its dependencies
pub fn initialize(db_pool: PgPool, network: Arc<Network>) -> InvoicingModule {
    // Initialize infrastructure components
    let db_pool = Arc::new(db_pool);
    let invoice_repository = Arc::new(PgInvoiceRepository::new(db_pool.clone()));
    
    // Initialize P2P invoice sharing
    let local_peer_id = network.local_peer_id().clone();
    let p2p_sharing = Arc::new(P2PInvoiceSharing::new(network, local_peer_id));
    
    // For now, we'll create a simple router that doesn't have any specific routes
    let router = Router::new();
    
    let query = InvoicingQuery;
    let mutation = InvoicingMutation;
    let subscription = InvoicingSubscription;

    InvoicingModule {
        router,
        query,
        mutation,
        subscription,
        p2p_sharing,
    }
}