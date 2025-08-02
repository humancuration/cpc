use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Tip {
    pub id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub course_id: Option<Uuid>,
    pub amount: f64,
    pub currency: String,
    pub created_at: DateTime<Utc>,
}

impl Tip {
    pub fn new(from_user_id: Uuid, to_user_id: Uuid, amount: f64, currency: String, course_id: Option<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            from_user_id,
            to_user_id,
            course_id,
            amount,
            currency,
            created_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_tip_creation() {
        let from_user_id = Uuid::new_v4();
        let to_user_id = Uuid::new_v4();
        let amount = 10.0;
        let currency = "USD".to_string();
        let course_id = Some(Uuid::new_v4());
        
        let tip = Tip::new(from_user_id, to_user_id, amount, currency.clone(), course_id);
        
        assert_eq!(tip.from_user_id, from_user_id);
        assert_eq!(tip.to_user_id, to_user_id);
        assert_eq!(tip.amount, amount);
        assert_eq!(tip.currency, currency);
        assert_eq!(tip.course_id, course_id);
        assert!(tip.created_at <= Utc::now());
    }

    #[test]
    fn test_tip_creation_without_course() {
        let from_user_id = Uuid::new_v4();
        let to_user_id = Uuid::new_v4();
        let amount = 5.0;
        let currency = "EUR".to_string();
        
        let tip = Tip::new(from_user_id, to_user_id, amount, currency.clone(), None);
        
        assert_eq!(tip.from_user_id, from_user_id);
        assert_eq!(tip.to_user_id, to_user_id);
        assert_eq!(tip.amount, amount);
        assert_eq!(tip.currency, currency);
        assert_eq!(tip.course_id, None);
        assert!(tip.created_at <= Utc::now());
    }
}