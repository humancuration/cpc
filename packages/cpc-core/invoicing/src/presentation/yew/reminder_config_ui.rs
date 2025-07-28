//! Yew UI components for payment reminder configuration
//!
//! This module contains the Yew components for configuring automatic payment reminders.

use yew::prelude::*;
use crate::domain::reminder::PaymentReminderConfig;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Properties for the ReminderConfigForm component
#[derive(Properties, PartialEq)]
pub struct ReminderConfigFormProps {
    pub config: Option<PaymentReminderConfig>,
    pub on_save: Callback<PaymentReminderConfig>,
    pub on_cancel: Callback<()>,
}

/// State for the ReminderConfigForm component
#[derive(Clone, PartialEq)]
struct ConfigFormState {
    enabled: bool,
    first_reminder_days: i32,
    repeat_reminder_days: i32,
    max_reminders: u32,
    reminder_template: String,
}

/// Component to configure payment reminders
#[function_component(ReminderConfigForm)]
pub fn reminder_config_form(props: &ReminderConfigFormProps) -> Html {
    let state = use_state(|| {
        if let Some(config) = &props.config {
            ConfigFormState {
                enabled: config.enabled,
                first_reminder_days: config.first_reminder_days,
                repeat_reminder_days: config.repeat_reminder_days,
                max_reminders: config.max_reminders,
                reminder_template: config.reminder_template.clone(),
            }
        } else {
            // Default values
            ConfigFormState {
                enabled: true,
                first_reminder_days: 3,
                repeat_reminder_days: 7,
                max_reminders: 3,
                reminder_template: "This is a reminder that your invoice #{invoice_id} is due on {due_date}. Please make payment at your earliest convenience.".to_string(),
            }
        }
    });

    let on_enabled_change = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut new_state = (*state).clone();
            new_state.enabled = input.checked();
            state.set(new_state);
        })
    };

    let on_first_reminder_days_input = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let value = input.value().parse().unwrap_or(3);
            let mut new_state = (*state).clone();
            new_state.first_reminder_days = value;
            state.set(new_state);
        })
    };

    let on_repeat_reminder_days_input = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let value = input.value().parse().unwrap_or(7);
            let mut new_state = (*state).clone();
            new_state.repeat_reminder_days = value;
            state.set(new_state);
        })
    };

    let on_max_reminders_input = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let value = input.value().parse().unwrap_or(3);
            let mut new_state = (*state).clone();
            new_state.max_reminders = value;
            state.set(new_state);
        })
    };

    let on_reminder_template_input = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut new_state = (*state).clone();
            new_state.reminder_template = input.value();
            state.set(new_state);
        })
    };

    let on_save = {
        let state = state.clone();
        let on_save = props.on_save.clone();
        let config_id = props.config.as_ref().map(|c| c.id);
        let user_id = props.config.as_ref().map(|c| c.user_id).unwrap_or_else(Uuid::new_v4);
        let created_at = props.config.as_ref().map(|c| c.created_at).unwrap_or_else(Utc::now);
        let updated_at = Utc::now();
        
        Callback::from(move |_| {
            let new_state = (*state).clone();
            
            let config = PaymentReminderConfig {
                id: config_id.unwrap_or_else(Uuid::new_v4),
                user_id,
                enabled: new_state.enabled,
                first_reminder_days: new_state.first_reminder_days,
                repeat_reminder_days: new_state.repeat_reminder_days,
                max_reminders: new_state.max_reminders,
                reminder_template: new_state.reminder_template.clone(),
                created_at,
                updated_at,
            };
            
            on_save.emit(config);
        })
    };

    let on_cancel = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |_| on_cancel.emit(()))
    };

    html! {
        <div class="reminder-config-form">
            <h2>
                {if props.config.is_some() { "Edit Reminder Configuration" } else { "Create Reminder Configuration" }}
            </h2>
            <form>
                <div class="form-group">
                    <label for="enabled">
                        <input
                            type="checkbox"
                            id="enabled"
                            checked={state.enabled}
                            onchange={on_enabled_change}
                        />
                        {" Enable automatic payment reminders"}
                    </label>
                </div>
                
                <div class="form-group">
                    <label for="first_reminder_days">{"Days before due date for first reminder"}</label>
                    <input
                        type="number"
                        id="first_reminder_days"
                        value={state.first_reminder_days.to_string()}
                        oninput={on_first_reminder_days_input}
                        min="1"
                        step="1"
                    />
                </div>
                
                <div class="form-group">
                    <label for="repeat_reminder_days">{"Days between repeat reminders"}</label>
                    <input
                        type="number"
                        id="repeat_reminder_days"
                        value={state.repeat_reminder_days.to_string()}
                        oninput={on_repeat_reminder_days_input}
                        min="1"
                        step="1"
                    />
                </div>
                
                <div class="form-group">
                    <label for="max_reminders">{"Maximum number of reminders"}</label>
                    <input
                        type="number"
                        id="max_reminders"
                        value={state.max_reminders.to_string()}
                        oninput={on_max_reminders_input}
                        min="1"
                        max="10"
                        step="1"
                    />
                </div>
                
                <div class="form-group">
                    <label for="reminder_template">{"Reminder message template"}</label>
                    <textarea
                        id="reminder_template"
                        value={state.reminder_template.clone()}
                        oninput={on_reminder_template_input}
                        placeholder="Enter reminder message template"
                        rows="5"
                    />
                    <div class="template-help">
                        <p>{"Available placeholders:"}</p>
                        <ul>
                            <li>{"{invoice_id} - Invoice ID"}</li>
                            <li>{"{due_date} - Due date"}</li>
                            <li>{"{client_name} - Client name"}</li>
                            <li>{"{total_amount} - Total amount"}</li>
                        </ul>
                    </div>
                </div>
                
                <div class="form-actions">
                    <button type="button" class="btn btn-primary" onclick={on_save}>
                        {"Save"}
                    </button>
                    <button type="button" class="btn btn-secondary" onclick={on_cancel}>
                        {"Cancel"}
                    </button>
                </div>
            </form>
        </div>
    }
}

/// Properties for the ReminderConfigList component
#[derive(Properties, PartialEq)]
pub struct ReminderConfigListProps {
    pub configs: Vec<PaymentReminderConfig>,
    pub on_edit: Callback<Uuid>,
    pub on_delete: Callback<Uuid>,
}

/// Component to display a list of reminder configurations
#[function_component(ReminderConfigList)]
pub fn reminder_config_list(props: &ReminderConfigListProps) -> Html {
    html! {
        <div class="reminder-config-list">
            <div class="header">
                <h2>{"Payment Reminder Configurations"}</h2>
            </div>
            <div class="configs">
                {for props.configs.iter().map(|config| {
                    html! {
                        <ReminderConfigItem 
                            config={config.clone()} 
                            on_edit={props.on_edit.clone()}
                            on_delete={props.on_delete.clone()}
                        />
                    }
                })}
            </div>
        </div>
    }
}

/// Properties for the ReminderConfigItem component
#[derive(Properties, PartialEq)]
pub struct ReminderConfigItemProps {
    pub config: PaymentReminderConfig,
    pub on_edit: Callback<Uuid>,
    pub on_delete: Callback<Uuid>,
}

/// Component to display a single reminder configuration
#[function_component(ReminderConfigItem)]
pub fn reminder_config_item(props: &ReminderConfigItemProps) -> Html {
    let config = &props.config;
    
    let on_edit = {
        let on_edit = props.on_edit.clone();
        let config_id = config.id;
        Callback::from(move |_| on_edit.emit(config_id))
    };
    
    let on_delete = {
        let on_delete = props.on_delete.clone();
        let config_id = config.id;
        Callback::from(move |_| on_delete.emit(config_id))
    };

    html! {
        <div class="config-item">
            <div class="config-header">
                <h3>{"Reminder Configuration"}</h3>
                <div class="config-status">
                    {if config.enabled { "Enabled" } else { "Disabled" }}
                </div>
            </div>
            <div class="config-details">
                <p><strong>{"First reminder:"}</strong> {format!("{} days before due date", config.first_reminder_days)}</p>
                <p><strong>{"Repeat every:"}</strong> {format!("{} days", config.repeat_reminder_days)}</p>
                <p><strong>{"Maximum reminders:"}</strong> {config.max_reminders}</p>
                <div class="template-preview">
                    <strong>{"Template preview:"}</strong>
                    <p>{&config.reminder_template}</p>
                </div>
            </div>
            <div class="config-actions">
                <button class="btn btn-secondary" onclick={on_edit}>{"Edit"}</button>
                <button class="btn btn-danger" onclick={on_delete}>{"Delete"}</button>
            </div>
        </div>
    }
}