use async_graphql::*;
use async_stream::stream;
use futures_util::stream::Stream;
use std::sync::Arc;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::Receiver as BroadcastReceiver;
use cpc_core::finance::royalty_service::RoyaltyService;

pub struct FinanceSubscription;

#[Subscription]
impl FinanceSubscription {
    async fn royalty_distribution_status(
        &self,
        ctx: &Context<'_>,
    ) -> Result<impl Stream<Item = crate::graphql::finance::types::RoyaltyDistributionStatus>> {
        let rx = ctx.data_unchecked::<BroadcastReceiver<cpc_core::finance::royalty_service::RoyaltyDistributionStatus>>();
        let mut rx = rx.resubscribe();
        
        Ok(stream! {
            loop {
                match rx.recv().await {
                    Ok(status) => {
                        yield crate::graphql::finance::types::RoyaltyDistributionStatus {
                            work_id: status.work_id,
                            status: status.status,
                            recipients: status.recipients as i32,
                        };
                    }
                    Err(RecvError::Lagged(_)) => {
                        // Handle lagged messages gracefully
                        continue;
                    }
                    Err(_) => {
                        // Channel closed
                        break;
                    }
                }
            }
        })
    }
}