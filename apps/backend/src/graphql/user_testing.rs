use crate::database::DbExecutor;
use crate::graphql::schema::Context;
use actix::Handler;
use juniper::{FieldError, FieldResult};
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug)]
pub struct FeedbackInput {
    pub rating: i32,
    pub comment: String,
    pub contact_email: Option<String>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct FeedbackSubmission {
    pub rating: i32,
    pub comment: String,
    pub contact_email: Option<String>,
}

#[derive(juniper::GraphQLObject)]
pub struct FeedbackResponse {
    pub success: bool,
    pub message: String,
}

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    async fn submit_feedback(
        context: &Context,
        input: FeedbackSubmission,
    ) -> FieldResult<FeedbackResponse> {
        // Validate input
        if input.rating < 1 || input.rating > 5 {
            return Err(FieldError::from("Rating must be between 1 and 5"));
        }
        if input.comment.len() > 1000 {
            return Err(FieldError::from("Comment must be less than 1000 characters"));
        }

        let db = context.db.clone();
        let feedback_input = FeedbackInput {
            rating: input.rating,
            comment: input.comment.clone(),
            contact_email: input.contact_email.clone(),
        };

        // Send feedback to database
        match db.send(feedback_input).await {
            Ok(_) => {
                // Log notification event (to be enhanced with proper notification service later)
                log::info!("Feedback submitted successfully. Triggering notification...");
                // TODO: Replace with actual notification service implementation
                
                Ok(FeedbackResponse {
                    success: true,
                    message: "Feedback submitted successfully".to_string(),
                })
            }
            Err(e) => {
                Err(FieldError::from(format!("Failed to submit feedback: {}", e)))
            }
        }
    }
}

impl Handler<FeedbackInput> for DbExecutor {
    type Result = Result<(), diesel::result::Error>;

    fn handle(&mut self, msg: FeedbackInput, _: &mut Self::Context) -> Self::Result {
        use crate::schema::user_feedback::dsl::*;
        use diesel::prelude::*;

        let conn = self.0.get().expect("Couldn't get db connection");

        diesel::insert_into(user_feedback)
            .values((
                rating.eq(msg.rating),
                comment.eq(msg.comment),
                contact_email.eq(msg.contact_email),
                created_at.eq(Utc::now().naive_utc()),
            ))
            .execute(&conn)?;

        Ok(())
    }
}