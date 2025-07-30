//! Mobile toolbar component for Finance-Sheets
//!
//! This component provides a context-aware action bar optimized for mobile devices
//! with touch-friendly controls and adaptive layout.

use yew::prelude::*;
use stylist::yew::use_style;
use stylist::Style;

/// Properties for the mobile toolbar component
#[derive(Properties, PartialEq)]
pub struct MobileToolbarProps {
    /// Toolbar title
    #[prop_or_default]
    pub title: String,
    
    /// Callback for back button
    #[prop_or_default]
    pub on_back: Callback<()>,
    
    /// Callback for action button
    #[prop_or_default]
    pub on_action: Callback<()>,
    
    /// Action button label
    #[prop_or("Action".to_string())]
    pub action_label: String,
    
    /// Whether to show the back button
    #[prop_or(true)]
    pub show_back: bool,
    
    /// Whether to show the action button
    #[prop_or(true)]
    pub show_action: bool,
    
    /// Additional actions to display
    #[prop_or_default]
    pub additional_actions: Vec<ToolbarAction>,
}

/// Toolbar action definition
#[derive(Debug, Clone, PartialEq)]
pub struct ToolbarAction {
    /// Action icon or text
    pub label: String,
    
    /// Callback when action is triggered
    pub on_click: Callback<()>,
    
    /// Whether this is a primary action
    pub is_primary: bool,
}

/// State for the mobile toolbar component
#[derive(Debug, Clone, PartialEq)]
pub struct MobileToolbarState {
    /// Whether the toolbar is visible
    is_visible: bool,
}

/// Messages for the mobile toolbar component
#[derive(Debug, Clone)]
pub enum MobileToolbarMsg {
    /// Show the toolbar
    Show,
    
    /// Hide the toolbar
    Hide,
}

/// Mobile toolbar component
#[derive(Debug)]
pub struct MobileToolbar {
    /// Component state
    state: MobileToolbarState,
}

impl Component for MobileToolbar {
    type Message = MobileToolbarMsg;
    type Properties = MobileToolbarProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: MobileToolbarState {
                is_visible: true,
            },
        }
    }
    
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MobileToolbarMsg::Show => {
                self.state.is_visible = true;
                true
            }
            
            MobileToolbarMsg::Hide => {
                self.state.is_visible = false;
                true
            }
        }
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        if !self.state.is_visible {
            return html! {};
        }
        
        let style = get_toolbar_styles();
        let props = ctx.props();
        let link = ctx.link();
        
        let on_back = link.callback(|_| MobileToolbarMsg::Hide);
        let on_action = link.callback(|_| MobileToolbarMsg::Show);
        
        html! {
            <div class={style}>
                <div class="mobile-toolbar">
                    if props.show_back {
                        <button 
                            class="toolbar-back-button"
                            onclick={props.on_back.reform(|_| ())}
                        >
                            {"←"}
                        </button>
                    }
                    
                    <div class="toolbar-title">
                        {&props.title}
                    </div>
                    
                    if props.show_action {
                        <button 
                            class="toolbar-action-button"
                            onclick={props.on_action.reform(|_| ())}
                        >
                            {&props.action_label}
                        </button>
                    }
                </div>
                
                if !props.additional_actions.is_empty() {
                    <div class="toolbar-actions">
                        {for props.additional_actions.iter().map(|action| {
                            let on_click = action.on_click.clone();
                            let class = if action.is_primary {
                                "toolbar-action-primary"
                            } else {
                                "toolbar-action-secondary"
                            };
                            
                            html! {
                                <button 
                                    class={classes!("toolbar-action-button", class)}
                                    onclick={move |_| on_click.emit(())}
                                >
                                    {&action.label}
                                </button>
                            }
                        })}
                    </div>
                }
            </div>
        }
    }
}

/// Get the CSS styles for the mobile toolbar
fn get_toolbar_styles() -> Style {
    use_style!(
        r#"
        .mobile-toolbar {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 16px;
            background-color: #007bff;
            color: white;
            height: 56px;
            box-sizing: border-box;
        }
        
        .toolbar-back-button {
            background: none;
            border: none;
            color: white;
            font-size: 24px;
            cursor: pointer;
            padding: 8px;
            min-width: 48px;
            min-height: 48px; /* Touch target optimization */
        }
        
        .toolbar-title {
            flex-grow: 1;
            text-align: center;
            font-size: 18px;
            font-weight: 500;
        }
        
        .toolbar-action-button {
            background: none;
            border: none;
            color: white;
            font-size: 16px;
            cursor: pointer;
            padding: 8px 16px;
            min-height: 48px; /* Touch target optimization */
            border-radius: 4px;
        }
        
        .toolbar-action-button:hover {
            background-color: rgba(255, 255, 255, 0.1);
        }
        
        .toolbar-actions {
            display: flex;
            padding: 8px 16px;
            background-color: #f8f9fa;
            border-bottom: 1px solid #dee2e6;
            overflow-x: auto;
        }
        
        .toolbar-action-button.toolbar-action-primary {
            background-color: #007bff;
            color: white;
        }
        
        .toolbar-action-button.toolbar-action-secondary {
            background-color: #6c757d;
            color: white;
        }
        
        .toolbar-action-button.toolbar-action-primary:hover,
        .toolbar-action-button.toolbar-action-secondary:hover {
            opacity: 0.8;
        }
    "#
    )
}