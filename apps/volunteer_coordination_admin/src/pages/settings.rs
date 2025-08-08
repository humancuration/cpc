use yew::prelude::*;
use stylist::{style, yew::styled_component};

#[styled_component(SettingsPage)]
pub fn settings_page() -> Html {
    let page_style = style!(
        r#"
        .settings-page {
            max-width: 800px;
            margin: 0 auto;
        }
        
        .page-header {
            margin-bottom: 30px;
        }
        
        .page-title {
            font-size: 2rem;
            color: #2c3e50;
            margin: 0 0 10px 0;
        }
        
        .page-description {
            color: #7f8c8d;
            margin: 0;
        }
        
        .settings-section {
            background: white;
            border-radius: 8px;
            box-shadow: 0 2px 8px rgba(0,0,0,0.1);
            padding: 20px;
            margin-bottom: 20px;
        }
        
        .section-title {
            font-size: 1.25rem;
            color: #34495e;
            margin: 0 0 15px 0;
            padding-bottom: 10px;
            border-bottom: 1px solid #ecf0f1;
        }
        
        .form-group {
            margin-bottom: 20px;
        }
        
        .form-label {
            display: block;
            margin-bottom: 5px;
            font-weight: 600;
            color: #2c3e50;
        }
        
        .form-control {
            width: 100%;
            padding: 10px;
            border: 1px solid #bdc3c7;
            border-radius: 4px;
            font-family: inherit;
            font-size: 1rem;
        }
        
        .form-control:focus {
            outline: none;
            border-color: #3498db;
            box-shadow: 0 0 0 2px rgba(52, 152, 219, 0.2);
        }
        
        .checkbox-group {
            display: flex;
            align-items: center;
        }
        
        .checkbox-group input {
            margin-right: 10px;
        }
        
        .btn {
            padding: 10px 20px;
            background-color: #3498db;
            color: white;
            border: none;
            border-radius: 4px;
            cursor: pointer;
            font-size: 1rem;
            transition: background-color 0.2s;
        }
        
        .btn:hover {
            background-color: #2980b9;
        }
        
        .btn-block {
            width: 100%;
        }
    "#
    ).unwrap();

    html! {
        <div class={page_style}>
            <div class="page-header">
                <h1 class="page-title">{"Settings"}</h1>
                <p class="page-description">{"Configure volunteer coordination dashboard preferences"}</p>
            </div>
            
            <div class="settings-section">
                <h2 class="section-title">{"Data Privacy & Consent"}</h2>
                <div class="form-group">
                    <label class="form-label">{"Data Collection Level"}</label>
                    <select class="form-control">
                        <option value="none">{"None - No data collection"}</option>
                        <option value="minimal">{"Minimal - Basic engagement metrics only"}</option>
                        <option value="standard" selected=true>{"Standard - Full engagement and feedback metrics"}</option>
                        <option value="enhanced">{"Enhanced - Additional demographic and behavioral data"}</option>
                    </select>
                </div>
                
                <div class="form-group">
                    <div class="checkbox-group">
                        <input type="checkbox" id="anonymous-data" checked=true />
                        <label for="anonymous-data">{"Anonymize all collected data"}</label>
                    </div>
                </div>
                
                <div class="form-group">
                    <div class="checkbox-group">
                        <input type="checkbox" id="community-sharing" checked=true />
                        <label for="community-sharing">{"Share anonymized insights with volunteer community"}</label>
                    </div>
                </div>
            </div>
            
            <div class="settings-section">
                <h2 class="section-title">{"Dashboard Preferences"}</h2>
                <div class="form-group">
                    <label class="form-label">{"Default Time Period"}</label>
                    <select class="form-control">
                        <option value="7">{"Last 7 days"}</option>
                        <option value="30" selected=true>{"Last 30 days"}</option>
                        <option value="90">{"Last 90 days"}</option>
                        <option value="365">{"Last year"}</option>
                    </select>
                </div>
                
                <div class="form-group">
                    <label class="form-label">{"Metrics Display"}</label>
                    <select class="form-control">
                        <option value="absolute" selected=true>{"Absolute values"}</option>
                        <option value="relative">{"Relative to baseline"}</option>
                        <option value="both">{"Both absolute and relative"}</option>
                    </select>
                </div>
                
                <div class="form-group">
                    <div class="checkbox-group">
                        <input type="checkbox" id="dark-mode" />
                        <label for="dark-mode">{"Enable dark mode"}</label>
                    </div>
                </div>
            </div>
            
            <div class="settings-section">
                <h2 class="section-title">{"Notification Settings"}</h2>
                <div class="form-group">
                    <div class="checkbox-group">
                        <input type="checkbox" id="email-notifications" checked=true />
                        <label for="email-notifications">{"Email notifications for significant changes"}</label>
                    </div>
                </div>
                
                <div class="form-group">
                    <div class="checkbox-group">
                        <input type="checkbox" id="slack-notifications" />
                        <label for="slack-notifications">{"Slack notifications for team"}</label>
                    </div>
                </div>
                
                <div class="form-group">
                    <label class="form-label">{"Alert Threshold"}</label>
                    <select class="form-control">
                        <option value="high">{"High - Only critical changes"}</option>
                        <option value="medium" selected=true>{"Medium - Significant changes"}</option>
                        <option value="low">{"Low - All notable changes"}</option>
                    </select>
                </div>
            </div>
            
            <button class="btn btn-block">{"Save Settings"}</button>
        </div>
    }
}