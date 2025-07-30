//! Searchable dropdown component
//!
//! This component provides a reusable searchable dropdown that can be used
//! with any list of items. It supports debounced search, keyboard navigation,
//! and ARIA accessibility.

use yew::prelude::*;
use stylist::yew::use_style;
use stylist::Style;
use gloo_timers::callback::Timeout;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;

/// Properties for the search dropdown component
#[derive(Properties, PartialEq, Clone)]
pub struct SearchDropdownProps<T: PartialEq + Clone + 'static> {
    /// Callback when an item is selected
    pub on_select: Callback<T>,
    
    /// List of items to display in the dropdown
    pub items: Vec<T>,
    
    /// Function to convert an item to a display string
    pub item_to_string: Callback<T, String>,
    
    /// Optional selected item
    #[prop_or(None)]
    pub selected: Option<T>,
    
    /// Placeholder text for the search input
    #[prop_or("Search...".to_string())]
    pub placeholder: String,
    
    /// ARIA label for accessibility
    #[prop_or("Searchable dropdown".to_string())]
    pub aria_label: String,
}

/// State for the search dropdown component
#[derive(Debug, Clone, PartialEq)]
pub struct SearchDropdownState<T: PartialEq + Clone> {
    /// Current search query
    search_query: String,
    
    /// Currently selected item
    selected_item: Option<T>,
    
    /// Whether the dropdown is open
    is_open: bool,
    
    /// Filtered items based on search query
    filtered_items: Vec<T>,
    
    /// Index of the currently highlighted item (for keyboard navigation)
    highlighted_index: Option<usize>,
}

/// Messages for the search dropdown component
#[derive(Debug, Clone)]
pub enum SearchDropdownMsg<T: PartialEq + Clone> {
    /// Update the search query
    SearchQueryChanged(String),
    
    /// Toggle the dropdown open/closed
    ToggleDropdown,
    
    /// Close the dropdown
    CloseDropdown,
    
    /// Select an item
    SelectItem(T),
    
    /// Highlight next item (arrow down)
    HighlightNext,
    
    /// Highlight previous item (arrow up)
    HighlightPrevious,
    
    /// Select highlighted item (enter)
    SelectHighlighted,
}

/// Searchable dropdown component
#[derive(Debug)]
pub struct SearchDropdown<T: PartialEq + Clone + 'static> {
    /// Component state
    state: SearchDropdownState<T>,
    
    /// Debounce timeout for search
    search_timeout: Option<Timeout>,
}

impl<T: PartialEq + Clone + 'static> Component for SearchDropdown<T> {
    type Message = SearchDropdownMsg<T>;
    type Properties = SearchDropdownProps<T>;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        let filtered_items = props.items.clone();
        
        Self {
            state: SearchDropdownState {
                search_query: String::new(),
                selected_item: props.selected.clone(),
                is_open: false,
                filtered_items,
                highlighted_index: None,
            },
            search_timeout: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SearchDropdownMsg::SearchQueryChanged(query) => {
                // Cancel any existing timeout
                self.search_timeout = None;
                
                // Update search query immediately
                self.state.search_query = query.clone();
                
                // Set a new timeout to filter items after 300ms
                let link = ctx.link().clone();
                self.search_timeout = Some(Timeout::new(300, move || {
                    link.send_message(SearchDropdownMsg::SearchQueryChanged(query.clone()));
                }));
                
                // Filter items immediately for better UX
                self.filter_items(ctx);
                
                true
            }
            
            SearchDropdownMsg::ToggleDropdown => {
                self.state.is_open = !self.state.is_open;
                if !self.state.is_open {
                    self.state.highlighted_index = None;
                }
                true
            }
            
            SearchDropdownMsg::CloseDropdown => {
                self.state.is_open = false;
                self.state.highlighted_index = None;
                true
            }
            
            SearchDropdownMsg::SelectItem(item) => {
                self.state.selected_item = Some(item.clone());
                self.state.is_open = false;
                self.state.highlighted_index = None;
                ctx.props().on_select.emit(item);
                true
            }
            
            SearchDropdownMsg::HighlightNext => {
                if self.state.is_open && !self.state.filtered_items.is_empty() {
                    let current_index = self.state.highlighted_index.unwrap_or(0);
                    let next_index = if current_index >= self.state.filtered_items.len() - 1 {
                        0
                    } else {
                        current_index + 1
                    };
                    self.state.highlighted_index = Some(next_index);
                    true
                } else {
                    false
                }
            }
            
            SearchDropdownMsg::HighlightPrevious => {
                if self.state.is_open && !self.state.filtered_items.is_empty() {
                    let current_index = self.state.highlighted_index.unwrap_or(0);
                    let prev_index = if current_index == 0 {
                        self.state.filtered_items.len() - 1
                    } else {
                        current_index - 1
                    };
                    self.state.highlighted_index = Some(prev_index);
                    true
                } else {
                    false
                }
            }
            
            SearchDropdownMsg::SelectHighlighted => {
                if let Some(index) = self.state.highlighted_index {
                    if index < self.state.filtered_items.len() {
                        let item = self.state.filtered_items[index].clone();
                        ctx.link().send_message(SearchDropdownMsg::SelectItem(item));
                    }
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let style = get_styles();
        
        let link = ctx.link();
        let props = ctx.props();
        
        let on_input_change = link.callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            SearchDropdownMsg::SearchQueryChanged(input.value())
        });
        
        let on_toggle = link.callback(|_| SearchDropdownMsg::ToggleDropdown);
        let on_blur = link.callback(|_| SearchDropdownMsg::CloseDropdown);
        
        let on_key_down = link.callback(|e: KeyboardEvent| {
            match e.key().as_str() {
                "ArrowDown" => {
                    e.prevent_default();
                    SearchDropdownMsg::HighlightNext
                }
                "ArrowUp" => {
                    e.prevent_default();
                    SearchDropdownMsg::HighlightPrevious
                }
                "Enter" => {
                    e.prevent_default();
                    SearchDropdownMsg::SelectHighlighted
                }
                "Escape" => {
                    e.prevent_default();
                    SearchDropdownMsg::CloseDropdown
                }
                _ => return SearchDropdownMsg::CloseDropdown, // This won't actually close, just a placeholder
            }
        });
        
        let selected_display = if let Some(item) = &self.state.selected_item {
            props.item_to_string.emit(item.clone())
        } else {
            "Select an option".to_string()
        };
        
        html! {
            <div class={style}>
                <div 
                    class="search-dropdown"
                    aria-label={props.aria_label.clone()}
                >
                    <div 
                        class="selected-display"
                        onclick={on_toggle}
                        tabindex="0"
                        role="button"
                        aria-haspopup="listbox"
                        aria-expanded={self.state.is_open.to_string()}
                    >
                        <span class="selected-text">{selected_display}</span>
                        <span class="dropdown-arrow">{"▼"}</span>
                    </div>
                    
                    if self.state.is_open {
                        <div class="dropdown-container">
                            <input
                                type="text"
                                class="search-input"
                                placeholder={props.placeholder.clone()}
                                value={self.state.search_query.clone()}
                                oninput={on_input_change}
                                onkeydown={on_key_down}
                                aria-autocomplete="list"
                                aria-controls="dropdown-list"
                            />
                            <ul 
                                class="dropdown-list" 
                                id="dropdown-list"
                                role="listbox"
                            >
                                {self.render_dropdown_items(ctx)}
                            </ul>
                        </div>
                    }
                </div>
            </div>
        }
    }
}

impl<T: PartialEq + Clone + 'static> SearchDropdown<T> {
    /// Filter items based on the search query
    fn filter_items(&mut self, ctx: &Context<Self>) {
        let props = ctx.props();
        let query = self.state.search_query.to_lowercase();
        
        if query.is_empty() {
            self.state.filtered_items = props.items.clone();
        } else {
            self.state.filtered_items = props.items
                .iter()
                .filter(|item| {
                    let item_str = props.item_to_string.emit((*item).clone());
                    item_str.to_lowercase().contains(&query)
                })
                .cloned()
                .collect();
        }
        
        // Reset highlighted index when filtering
        self.state.highlighted_index = if self.state.filtered_items.is_empty() {
            None
        } else {
            Some(0)
        };
    }
    
    /// Render the dropdown items
    fn render_dropdown_items(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let props = ctx.props();
        
        if self.state.filtered_items.is_empty() {
            html! {
                <li class="no-results">{"No results found"}</li>
            }
        } else {
            html! {
                <>
                    {for self.state.filtered_items.iter().enumerate().map(|(index, item)| {
                        let is_highlighted = self.state.highlighted_index == Some(index);
                        let item_str = props.item_to_string.emit(item.clone());
                        let on_click = link.callback(move |_| SearchDropdownMsg::SelectItem(item.clone()));
                        
                        html! {
                            <li
                                class={classes!("dropdown-item", is_highlighted.then(|| "highlighted"))}
                                onclick={on_click}
                                role="option"
                                aria-selected={if let Some(selected) = &self.state.selected_item {
                                    props.item_to_string.emit(selected.clone()) == item_str
                                } else {
                                    false
                                }}
                            >
                                {item_str}
                                if let Some(selected) = &self.state.selected_item {
                                    if props.item_to_string.emit(selected.clone()) == item_str {
                                        <span class="checkmark">{"✓"}</span>
                                    }
                                }
                            </li>
                        }
                    })}
                </>
            }
        }
    }
}

/// Get the CSS styles for the component
fn get_styles() -> Style {
    use_style!(
        r#"
        .search-dropdown {
            position: relative;
            width: 100%;
            font-family: Arial, sans-serif;
        }
        
        .selected-display {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 8px 12px;
            border: 1px solid #ccc;
            border-radius: 4px;
            background-color: white;
            cursor: pointer;
            min-height: 36px;
        }
        
        .selected-display:focus {
            outline: 2px solid #007bff;
            outline-offset: -2px;
        }
        
        .selected-text {
            flex-grow: 1;
            text-align: left;
        }
        
        .dropdown-arrow {
            margin-left: 8px;
            color: #666;
        }
        
        .dropdown-container {
            position: absolute;
            top: 100%;
            left: 0;
            right: 0;
            z-index: 1000;
            border: 1px solid #ccc;
            border-radius: 4px;
            background-color: white;
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
            margin-top: 4px;
        }
        
        .search-input {
            width: 100%;
            padding: 8px 12px;
            border: none;
            border-bottom: 1px solid #eee;
            border-radius: 4px 4px 0 0;
            box-sizing: border-box;
        }
        
        .search-input:focus {
            outline: none;
        }
        
        .dropdown-list {
            list-style: none;
            margin: 0;
            padding: 0;
            max-height: 200px;
            overflow-y: auto;
        }
        
        .dropdown-item {
            padding: 8px 12px;
            cursor: pointer;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        
        .dropdown-item:hover,
        .dropdown-item.highlighted {
            background-color: #f5f5f5;
        }
        
        .checkmark {
            color: #007bff;
            font-weight: bold;
        }
        
        .no-results {
            padding: 8px 12px;
            color: #666;
            font-style: italic;
        }
    "#
    )
}