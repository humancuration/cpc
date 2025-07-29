pub mod dashboard;
pub mod components;
pub mod hooks;
pub mod utils;

use yew::prelude::*;
use crate::context::auth::AuthProvider;

#[function_component(BusinessToolsApp)]
pub fn business_tools_app() -> Html {
    html! {
        <AuthProvider>
            <div class="business-tools-container">
                <dashboard::BusinessDashboard />
            </div>
        </AuthProvider>
    }
}
