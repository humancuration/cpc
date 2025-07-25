use yewdux::prelude::*;
use crate::types::AccountingDashboard;
use crate::types::impact::ImpactDashboardData;
use uuid::Uuid;
use cpc_core::accounting::PeriodType;

#[derive(Debug, Clone, PartialEq, Store)]
pub struct Store {
    pub dashboard: Option<AccountingDashboard>,
    pub impact: Option<ImpactDashboardData>,
    pub org_id: Uuid,
    pub current_period: PeriodType,
    pub loading: bool,
    pub error: Option<String>,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            dashboard: None,
            impact: None,
            org_id: Uuid::new_v4(), // TODO: Get from user context
            current_period: PeriodType::Current,
            loading: false,
            error: None,
        }
    }
}

#[derive(Debug)]
pub enum Action {
    SetDashboard(AccountingDashboard),
    SetImpactData(ImpactDashboardData),
    SetLoading(bool),
    SetError(Option<String>),
    SetPeriod(PeriodType),
}

impl Reducer<Store> for Action {
    fn apply(self, mut state: Rc<Store>) -> Rc<Store> {
        let state = Rc::make_mut(&mut state);
        
        match self {
            Action::SetDashboard(dashboard) => {
                state.dashboard = Some(dashboard);
                state.loading = false;
                state.error = None;
            }
            Action::SetImpactData(data) => {
                state.impact = Some(data);
                state.loading = false;
                state.error = None;
            }
            Action::SetLoading(loading) => {
                state.loading = loading;
            }
            Action::SetError(error) => {
                state.error = error;
                state.loading = false;
            }
            Action::SetPeriod(period) => {
                state.current_period = period;
            }
        }
        
        state.clone().into()
    }
}

// Convenience functions for use in components
pub fn set_dashboard_data(dashboard: AccountingDashboard) -> impl Reducer<Store> {
    move |_| Action::SetDashboard(dashboard.clone())
}

pub fn set_loading(loading: bool) -> impl Reducer<Store> {
    move |_| Action::SetLoading(loading)
}

pub fn set_error(error: Option<String>) -> impl Reducer<Store> {
    move |_| Action::SetError(error.clone())
}

pub fn set_period(period: PeriodType) -> impl Reducer<Store> {
    move |_| Action::SetPeriod(period.clone())
}

pub fn set_impact_data(data: ImpactDashboardData) -> impl Reducer<Store> {
    move |_| Action::SetImpactData(data.clone())
}