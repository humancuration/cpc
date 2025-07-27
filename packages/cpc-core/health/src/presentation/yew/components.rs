//! Yew web components for the health module
//!
//! This module provides web-based UI components for health data management.

use yew::prelude::*;
use crate::domain::vital_signs::{VitalSign, VitalSignType};
use crate::domain::health_condition::{HealthCondition, ConditionSeverity, ConditionStatus};

/// Props for the vital sign list component
#[derive(Properties, PartialEq)]
pub struct VitalSignListProps {
    pub vital_signs: Vec<VitalSign>,
}

/// Component to display a list of vital signs
#[function_component(VitalSignList)]
pub fn vital_sign_list(props: &VitalSignListProps) -> Html {
    html! {
        <div class="vital-signs-list">
            <h2>{"Vital Signs"}</h2>
            <ul>
                {for props.vital_signs.iter().map(|vital_sign| {
                    html! {
                        <li>
                            <VitalSignItem vital_sign={vital_sign.clone()} />
                        </li>
                    }
                })}
            </ul>
        </div>
    }
}

/// Props for the vital sign item component
#[derive(Properties, PartialEq)]
pub struct VitalSignItemProps {
    pub vital_sign: VitalSign,
}

/// Component to display a single vital sign
#[function_component(VitalSignItem)]
pub fn vital_sign_item(props: &VitalSignItemProps) -> Html {
    let vital_sign = &props.vital_sign;
    
    html! {
        <div class="vital-sign-item">
            <span class="measurement-type">
                {format!("{:?}", vital_sign.measurement_type)}
            </span>
            <span class="value">
                {format!("{} {}", vital_sign.value, vital_sign.unit)}
            </span>
            <span class="timestamp">
                {vital_sign.timestamp.format("%Y-%m-%d %H:%M:%S").to_string()}
            </span>
        </div>
    }
}

/// Props for the health condition list component
#[derive(Properties, PartialEq)]
pub struct HealthConditionListProps {
    pub conditions: Vec<HealthCondition>,
}

/// Component to display a list of health conditions
#[function_component(HealthConditionList)]
pub fn health_condition_list(props: &HealthConditionListProps) -> Html {
    html! {
        <div class="health-conditions-list">
            <h2>{"Health Conditions"}</h2>
            <ul>
                {for props.conditions.iter().map(|condition| {
                    html! {
                        <li>
                            <HealthConditionItem condition={condition.clone()} />
                        </li>
                    }
                })}
            </ul>
        </div>
    }
}

/// Props for the health condition item component
#[derive(Properties, PartialEq)]
pub struct HealthConditionItemProps {
    pub condition: HealthCondition,
}

/// Component to display a single health condition
#[function_component(HealthConditionItem)]
pub fn health_condition_item(props: &HealthConditionItemProps) -> Html {
    let condition = &props.condition;
    
    html! {
        <div class="health-condition-item">
            <span class="condition-type">
                {match &condition.condition_type {
                    crate::domain::health_condition::ConditionType::Chronic(desc) => format!("Chronic: {}", desc),
                    crate::domain::health_condition::ConditionType::Acute(desc) => format!("Acute: {}", desc),
                    crate::domain::health_condition::ConditionType::Genetic(desc) => format!("Genetic: {}", desc),
                    crate::domain::health_condition::ConditionType::MentalHealth(desc) => format!("Mental Health: {}", desc),
                }}
            </span>
            <span class="severity">
                {format!("Severity: {:?}", condition.severity)}
            </span>
            <span class="status">
                {format!("Status: {:?}", condition.status)}
            </span>
            <span class="diagnosis-date">
                {condition.diagnosis_date.format("%Y-%m-%d").to_string()}
            </span>
        </div>
    }
}

/// Props for the health dashboard component
#[derive(Properties, PartialEq)]
pub struct HealthDashboardProps {
    pub vital_signs: Vec<VitalSign>,
    pub conditions: Vec<HealthCondition>,
}

/// Component for the main health dashboard
#[function_component(HealthDashboard)]
pub fn health_dashboard(props: &HealthDashboardProps) -> Html {
    html! {
        <div class="health-dashboard">
            <h1>{"Health Dashboard"}</h1>
            <div class="dashboard-content">
                <VitalSignList vital_signs={props.vital_signs.clone()} />
                <HealthConditionList conditions={props.conditions.clone()} />
            </div>
        </div>
    }
}

/// Props for the vital sign input form
#[derive(Properties, PartialEq)]
pub struct VitalSignFormProps {
    pub on_submit: Callback<VitalSignFormData>,
}

/// Form data for vital sign input
#[derive(Debug, Clone, PartialEq)]
pub struct VitalSignFormData {
    pub measurement_type: VitalSignType,
    pub value: f32,
    pub unit: String,
    pub notes: Option<String>,
}

/// Component for entering new vital signs
#[function_component(VitalSignForm)]
pub fn vital_sign_form(props: &VitalSignFormProps) -> Html {
    let measurement_type = use_state(|| VitalSignType::HeartRate);
    let value = use_state(|| 0.0);
    let unit = use_state(|| "bpm".to_string());
    let notes = use_state(|| None);

    let onsubmit = {
        let on_submit = props.on_submit.clone();
        let measurement_type = measurement_type.clone();
        let value = value.clone();
        let unit = unit.clone();
        let notes = notes.clone();
        
        Callback::from(move |_| {
            let form_data = VitalSignFormData {
                measurement_type: (*measurement_type).clone(),
                value: *value,
                unit: unit.to_string(),
                notes: notes.as_ref().cloned(),
            };
            on_submit.emit(form_data);
        })
    };

    html! {
        <form class="vital-sign-form" onsubmit={onsubmit}>
            <h2>{"Record Vital Sign"}</h2>
            <div class="form-group">
                <label for="measurement-type">{"Measurement Type:"}</label>
                <select id="measurement-type" onchange={
                    let measurement_type = measurement_type.clone();
                    Callback::from(move |e: Event| {
                        let selected = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                        let new_type = match selected.as_str() {
                            "HeartRate" => VitalSignType::HeartRate,
                            "BloodPressure" => VitalSignType::BloodPressure,
                            "BloodGlucose" => VitalSignType::BloodGlucose,
                            "BodyTemperature" => VitalSignType::BodyTemperature,
                            "OxygenSaturation" => VitalSignType::OxygenSaturation,
                            "RespiratoryRate" => VitalSignType::RespiratoryRate,
                            "BodyWeight" => VitalSignType::BodyWeight,
                            "BodyMassIndex" => VitalSignType::BodyMassIndex,
                            _ => VitalSignType::HeartRate,
                        };
                        measurement_type.set(new_type);
                    })
                }>
                    <option value="HeartRate">{"Heart Rate"}</option>
                    <option value="BloodPressure">{"Blood Pressure"}</option>
                    <option value="BloodGlucose">{"Blood Glucose"}</option>
                    <option value="BodyTemperature">{"Body Temperature"}</option>
                    <option value="OxygenSaturation">{"Oxygen Saturation"}</option>
                    <option value="RespiratoryRate">{"Respiratory Rate"}</option>
                    <option value="BodyWeight">{"Body Weight"}</option>
                    <option value="BodyMassIndex">{"Body Mass Index"}</option>
                </select>
            </div>
            <div class="form-group">
                <label for="value">{"Value:"}</label>
                <input type="number" id="value" step="0.1" oninput={
                    let value = value.clone();
                    Callback::from(move |e: InputEvent| {
                        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                        if let Ok(val) = input.value().parse::<f32>() {
                            value.set(val);
                        }
                    })
                } />
            </div>
            <div class="form-group">
                <label for="unit">{"Unit:"}</label>
                <input type="text" id="unit" value={unit.to_string()} oninput={
                    let unit = unit.clone();
                    Callback::from(move |e: InputEvent| {
                        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                        unit.set(input.value());
                    })
                } />
            </div>
            <div class="form-group">
                <label for="notes">{"Notes:"}</label>
                <textarea id="notes" oninput={
                    let notes = notes.clone();
                    Callback::from(move |e: InputEvent| {
                        let input = e.target_unchecked_into::<web_sys::HtmlTextAreaElement>();
                        notes.set(Some(input.value()));
                    })
                }></textarea>
            </div>
            <button type="submit">{"Record"}</button>
        </form>
    }
}