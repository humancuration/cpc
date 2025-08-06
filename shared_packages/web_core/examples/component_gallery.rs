//! Component gallery example
//!
//! This example demonstrates all the available components in the web_core library.

use web_core::components::{
    Button, ButtonVariant,
    Checkbox,
    ErrorBoundary,
    Form, FormSubmitEvent,
    Modal,
    RadioButton,
    Select, SelectOption,
    TextArea, TextAreaResize,
    TextInput, InputType,
};
use web_core::utils::error_handling::WebError;
use yew::prelude::*;

/// Main application component
pub struct ComponentGallery {
    modal_open: bool,
    checkbox_checked: bool,
    text_input_value: String,
    text_area_value: String,
    select_value: String,
    radio_value: String,
    error_boundary_has_error: bool,
}

pub enum Msg {
    ToggleModal,
    ToggleCheckbox,
    UpdateTextInput(String),
    UpdateTextArea(String),
    UpdateSelect(String),
    UpdateRadio(String),
    SubmitForm(FormSubmitEvent),
    ResetForm,
    TriggerError,
    HandleError(WebError),
}

impl Component for ComponentGallery {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            modal_open: false,
            checkbox_checked: false,
            text_input_value: String::new(),
            text_area_value: String::new(),
            select_value: String::new(),
            radio_value: String::new(),
            error_boundary_has_error: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleModal => {
                self.modal_open = !self.modal_open;
                true
            }
            Msg::ToggleCheckbox => {
                self.checkbox_checked = !self.checkbox_checked;
                true
            }
            Msg::UpdateTextInput(value) => {
                self.text_input_value = value;
                true
            }
            Msg::UpdateTextArea(value) => {
                self.text_area_value = value;
                true
            }
            Msg::UpdateSelect(value) => {
                self.select_value = value;
                true
            }
            Msg::UpdateRadio(value) => {
                self.radio_value = value;
                true
            }
            Msg::SubmitForm(event) => {
                web_sys::console::log_1(&format!("Form submitted with data: {:?}", event.data).into());
                true
            }
            Msg::ResetForm => {
                self.text_input_value = String::new();
                self.text_area_value = String::new();
                self.select_value = String::new();
                self.radio_value = String::new();
                self.checkbox_checked = false;
                true
            }
            Msg::TriggerError => {
                self.error_boundary_has_error = true;
                true
            }
            Msg::HandleError(error) => {
                web_sys::console::error_1(&format!("Error caught by boundary: {:?}", error).into());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="component-gallery">
                <h1>{"Web Core Component Gallery"}</h1>
                
                <section>
                    <h2>{"Buttons"}</h2>
                    <div class="button-examples">
                        <Button variant={ButtonVariant::Primary}>
                            {"Primary Button"}
                        </Button>
                        <Button variant={ButtonVariant::Secondary}>
                            {"Secondary Button"}
                        </Button>
                        <Button variant={ButtonVariant::Danger}>
                            {"Danger Button"}
                        </Button>
                        <Button variant={ButtonVariant::Text}>
                            {"Text Button"}
                        </Button>
                    </div>
                </section>
                
                <section>
                    <h2>{"Radio Buttons"}</h2>
                    <div class="form-examples">
                        <RadioButton
                            name="radio-group"
                            value="option1"
                            checked={self.radio_value == "option1"}
                            onchange={ctx.link().callback(|checked: bool| {
                                if checked { Msg::UpdateRadio("option1".to_string()) } else { Msg::UpdateRadio(String::new()) }
                            })}
                            label="Option 1"
                        />
                        <RadioButton
                            name="radio-group"
                            value="option2"
                            checked={self.radio_value == "option2"}
                            onchange={ctx.link().callback(|checked: bool| {
                                if checked { Msg::UpdateRadio("option2".to_string()) } else { Msg::UpdateRadio(String::new()) }
                            })}
                            label="Option 2"
                        />
                        <RadioButton
                            name="radio-group"
                            value="option3"
                            checked={self.radio_value == "option3"}
                            onchange={ctx.link().callback(|checked: bool| {
                                if checked { Msg::UpdateRadio("option3".to_string()) } else { Msg::UpdateRadio(String::new()) }
                            })}
                            label="Option 3"
                        />
                    </div>
                </section>
                
                <section>
                    <h2>{"Form"}</h2>
                    <div class="form-examples">
                        <Form
                            onsubmit={ctx.link().callback(Msg::SubmitForm)}
                            onreset={ctx.link().callback(|_| Msg::ResetForm)}
                        >
                            <TextInput
                                value={self.text_input_value.clone()}
                                onchange={ctx.link().callback(Msg::UpdateTextInput)}
                                placeholder="Enter text here"
                            />
                            <TextArea
                                value={self.text_area_value.clone()}
                                onchange={ctx.link().callback(Msg::UpdateTextArea)}
                                placeholder="Enter multi-line text here"
                                rows={3}
                            />
                            <Select
                                value={self.select_value.clone()}
                                onchange={ctx.link().callback(Msg::UpdateSelect)}
                                placeholder="Choose an option"
                                options={vec![
                                    SelectOption { value: "option1".to_string(), label: "Option 1".to_string(), disabled: false },
                                    SelectOption { value: "option2".to_string(), label: "Option 2".to_string(), disabled: false },
                                    SelectOption { value: "option3".to_string(), label: "Option 3".to_string(), disabled: true },
                                ]}
                            />
                            <div>
                                <RadioButton
                                    name="form-radio-group"
                                    value="option1"
                                    checked={self.radio_value == "option1"}
                                    onchange={ctx.link().callback(|checked: bool| {
                                        if checked { Msg::UpdateRadio("option1".to_string()) } else { Msg::UpdateRadio(String::new()) }
                                    })}
                                    label="Option 1"
                                />
                                <RadioButton
                                    name="form-radio-group"
                                    value="option2"
                                    checked={self.radio_value == "option2"}
                                    onchange={ctx.link().callback(|checked: bool| {
                                        if checked { Msg::UpdateRadio("option2".to_string()) } else { Msg::UpdateRadio(String::new()) }
                                    })}
                                    label="Option 2"
                                />
                            </div>
                            <Checkbox
                                checked={self.checkbox_checked}
                                onchange={ctx.link().callback(|_| Msg::ToggleCheckbox)}
                                label="Check me"
                            />
                            <div>
                                <Button type="submit">{"Submit"}</Button>
                                <Button type="reset">{"Reset"}</Button>
                            </div>
                        </Form>
                    </div>
                </section>
                
                <section>
                    <h2>{"Select"}</h2>
                    <div class="form-examples">
                        <Select
                            value={self.select_value.clone()}
                            onchange={ctx.link().callback(Msg::UpdateSelect)}
                            placeholder="Choose an option"
                            options={vec![
                                SelectOption { value: "option1".to_string(), label: "Option 1".to_string(), disabled: false },
                                SelectOption { value: "option2".to_string(), label: "Option 2".to_string(), disabled: false },
                                SelectOption { value: "option3".to_string(), label: "Option 3".to_string(), disabled: true },
                            ]}
                        />
                    </div>
                </section>
                
                <section>
                    <h2>{"Text Area"}</h2>
                    <div class="form-examples">
                        <TextArea
                            value={self.text_area_value.clone()}
                            onchange={ctx.link().callback(Msg::UpdateTextArea)}
                            placeholder="Enter multi-line text here"
                            rows={4}
                        />
                    </div>
                </section>
                
                <section>
                    <h2>{"Form Controls"}</h2>
                    <div class="form-examples">
                        <TextInput
                            value={self.text_input_value.clone()}
                            onchange={ctx.link().callback(Msg::UpdateTextInput)}
                            placeholder="Enter text here"
                        />
                        
                        <Checkbox
                            checked={self.checkbox_checked}
                            onchange={ctx.link().callback(|_| Msg::ToggleCheckbox)}
                            label="Check me"
                        />
                    </div>
                </section>
                
                <section>
                    <h2>{"Modal"}</h2>
                    <Button onclick={ctx.link().callback(|_| Msg::ToggleModal)}>
                        {"Open Modal"}
                    </Button>
                    
                    <Modal
                        open={self.modal_open}
                        onclose={ctx.link().callback(|_| Msg::ToggleModal)}
                        title="Example Modal"
                    >
                        <p>{"This is an example modal dialog."}</p>
                        <Button onclick={ctx.link().callback(|_| Msg::ToggleModal)}>
                            {"Close"}
                        </Button>
                    </Modal>
                </section>
                
                <section>
                    <h2>{"Error Boundary"}</h2>
                    <Button onclick={ctx.link().callback(|_| Msg::TriggerError)}>
                        {"Trigger Error"}
                    </Button>
                    
                    <ErrorBoundary on_error={ctx.link().callback(Msg::HandleError)}>
                        if self.error_boundary_has_error {
                            <div>{"This will cause an error"}</div>
                            // Intentional error for demonstration
                            // In a real app, this would be a component that might fail
                        } else {
                            <p>{"This content is wrapped in an error boundary. Click the button to trigger an error."}</p>
                        }
                    </ErrorBoundary>
                </section>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<ComponentGallery>::new().render();
}