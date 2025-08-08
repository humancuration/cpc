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
            padding: 25px;
            margin-bottom: 30px;
        }
        
        .section-title {
            font-size: 1.3rem;
            color: #34495e;
            margin: 0 0 20px 0;
            padding-bottom: 10px;
            border-bottom: 1px solid #ecf0f1;
        }
        
        .form-group {
            margin-bottom: 20px;
        }
        
        .form-label {
            display: block;
            margin-bottom: 5px;
            font-weight: bold;
            color: #2c3e50;
        }
        
        .form-input {
            width: 100%;
            padding: 10px;
            border: 1px solid #bdc3c7;
            border-radius: 4px;
            font-size: 1rem;
            box-sizing: border-box;
        }
        
        .form-input:focus {
            outline: none;
            border-color: #3498db;
            box-shadow: 0 0 0 2px rgba(52, 152, 219, 0.2);
        }
        
        .form-select {
            width: 100%;
            padding: 10px;
            border: 1px solid #bdc3c7;
            border-radius: 4px;
            font-size: 1rem;
            background-color: white;
            box-sizing: border-box;
        }
        
        .form-checkbox {
            margin-right: 8px;
        }
        
        .form-row {
            display: flex;
            gap: 15px;
        }
        
        .form-row .form-group {
            flex: 1;
        }
        
        .btn {
            padding: 10px 20px;
            border: none;
            border-radius: 4px;
            font-size: 1rem;
            cursor: pointer;
            transition: background-color 0.2s;
        }
        
        .btn-primary {
            background-color: #3498db;
            color: white;
        }
        
        .btn-primary:hover {
            background-color: #2980b9;
        }
        
        .btn-secondary {
            background-color: #95a5a6;
            color: white;
        }
        
        .btn-secondary:hover {
            background-color: #7f8c8d;
        }
        
        .form-actions {
            display: flex;
            gap: 10px;
            margin-top: 20px;
        }
        
        @media (max-width: 768px) {
            .form-row {
                flex-direction: column;
                gap: 0;
            }
        }
    "#
    ).unwrap();

    html! {
        <div class={page_style}>
            <div class="page-header">
                <h1 class="page-title">{"Settings"}</h1>
                <p class="page-description">{"Configure analytics and tracking preferences"}</p>
            </div>
            
            <div class="settings-section">
                <h2 class="section-title">{"Data Collection Preferences"}</h2>
                <div class="form-group">
                    <label class="form-label">{"Consent Level"}</label>
                    <select class="form-select">
                        <option value="none">{"None - No data collection"}</option>
                        <option value="minimal" selected=true>{"Minimal - Basic engagement metrics only"}</option>
                        <option value="standard">{"Standard - Full analytics and feedback"}</option>
                        <option value="enhanced">{"Enhanced - Detailed behavioral analysis"}</option>
                    </select>
                </div>
                
                <div class="form-group">
                    <label class="form-label">{"Data Retention Period"}</label>
                    <select class="form-select">
                        <option value="30">{"30 days"}</option>
                        <option value="90" selected=true>{"90 days"}</option>
                        <option value="365">{"1 year"}</option>
                        <option value="0">{"Indefinite"}</option>
                    </select>
                </div>
                
                <div class="form-group">
                    <label class="form-label">
                        <input type="checkbox" class="form-checkbox" checked=true />
                        {"Anonymize user data"}
                    </label>
                </div>
                
                <div class="form-group">
                    <label class="form-label">
                        <input type="checkbox" class="form-checkbox" checked=true />
                        {"Share aggregated insights with community"}
                    </label>
                </div>
            </div>
            
            <div class="settings-section">
                <h2 class="section-title">{"Analytics Configuration"}</h2>
                <div class="form-group">
                    <label class="form-label">{"Dashboard Refresh Interval"}</label>
                    <select class="form-select">
                        <option value="60">{"1 minute"}</option>
                        <option value="300" selected=true>{"5 minutes"}</option>
                        <option value="600">{"10 minutes"}</option>
                        <option value="1800">{"30 minutes"}</option>
                        <option value="3600">{"1 hour"}</option>
                    </select>
                </div>
                
                <div class="form-group">
                    <label class="form-label">{"Metrics Calculation Window"}</label>
                    <select class="form-select">
                        <option value="7">{"7 days"}</option>
                        <option value="30" selected=true>{"30 days"}</option>
                        <option value="90">{"90 days"}</option>
                        <option value="365">{"1 year"}</option>
                    </select>
                </div>
                
                <div class="form-group">
                    <label class="form-label">
                        <input type="checkbox" class="form-checkbox" checked=true />
                        {"Enable real-time metrics"}
                    </label>
                </div>
            </div>
            
            <div class="settings-section">
                <h2 class="section-title">{"Notification Preferences"}</h2>
                <div class="form-group">
                    <label class="form-label">
                        <input type="checkbox" class="form-checkbox" checked=true />
                        {"Email reports (weekly)"}
                    </label>
                </div>
                
                <div class="form-group">
                    <label class="form-label">
                        <input type="checkbox" class="form-checkbox" />
                        {"Email reports (monthly)"}
                    </label>
                </div>
                
                <div class="form-group">
                    <label class="form-label">
                        <input type="checkbox" class="form-checkbox" checked=true />
                        {"In-app notifications for significant changes"}
                    </label>
                </div>
            </div>
            
            <div class="form-actions">
                <button class="btn btn-primary">{"Save Settings"}</button>
                <button class="btn btn-secondary">{"Reset to Defaults"}</button>
            </div>
        </div>
    }
}