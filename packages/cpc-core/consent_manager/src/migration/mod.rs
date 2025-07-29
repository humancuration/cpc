//! Migration utilities for converting existing consent data.

pub mod scm;
pub mod calendar;
pub mod finance;
pub mod crm;

pub use scm::migrate_scm_consent;
pub use calendar::migrate_calendar_consent;
pub use finance::migrate_finance_consent;
pub use crm::migrate_crm_consent;