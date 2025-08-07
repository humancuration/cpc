//! Block browser component for the Shtairir visual scripting system.
//!
//! This component provides a searchable, filterable interface for browsing
//! and selecting available code blocks.

/// Properties for the BlockBrowser component
#[derive(Properties, PartialEq)]
pub struct BlockBrowserProps {
    /// List of blocks to display
    pub blocks: Vec<BlockSpec>,
    
    /// Callback when a block is selected
    #[prop_or_default]
    pub on_block_select: Callback<BlockSpec>,
    
    /// Optional search filter
    #[prop_or_default]
    pub search_filter: Option<String>,
    
    /// Optional category filter
    #[prop_or_default]
    pub category_filter: Option<String>,
}

/// Block browser component for browsing Shtairir blocks
/// 
/// This component displays a searchable, filterable list of blocks
/// that can be selected for use in workflows.
#[function_component(BlockBrowser)]
pub fn block_browser(props: &BlockBrowserProps) -> Html {
    let search_term = use_state(|| props.search_filter.clone().unwrap_or_default());
    let selected_category = use_state(|| props.category_filter.clone().unwrap_or_default());
    
    // Filter blocks based on search and category
    let filtered_blocks = useMemo(
        (props.blocks.clone(), search_term.clone(), selected_category.clone()),
        |(blocks, search, category)| {
            blocks.iter().filter(|block| {
                let matches_search = search.is_empty() || 
                    block.name.to_lowercase().contains(&search.to_lowercase()) ||
                    block.title.to_lowercase().contains(&search.to_lowercase()) ||
                    block.description.to_lowercase().contains(&search.to_lowercase());
                
                let matches_category = category.is_empty() || 
                    block.categories.iter().any(|c| c == category);
                
                matches_search && matches_category
            }).cloned().collect::<Vec<BlockSpec>>()
        }
    );
    
    let on_search_input = {
        let search_term = search_term.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            search_term.set(input.value());
        })
    };
    
    let on_category_change = {
        let selected_category = selected_category.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            selected_category.set(select.value());
        })
    };
    
    // Get unique categories from blocks
    let categories = useMemo(props.blocks.clone(), |blocks| {
        let mut cats = std::collections::BTreeSet::new();
        for block in blocks {
            for category in &block.categories {
                cats.insert(category.clone());
            }
        }
        cats.into_iter().collect::<Vec<String>>()
    });
    
    html! {
        <div class="block-browser">
            <div class="block-browser-controls">
                <input
                    type="text"
                    placeholder="Search blocks..."
                    value={(*search_term).clone()}
                    oninput={on_search_input}
                    class="block-browser-search"
                />
                <select 
                    onchange={on_category_change} 
                    class="block-browser-category"
                    value={(*selected_category).clone()}
                >
                    <option value="">{"All Categories"}</option>
                    { for categories.iter().map(|category| {
                        html! {
                            <option value={category.clone()}>{category}</option>
                        }
                    })}
                </select>
            </div>
            
            <div class="block-browser-list">
                { for filtered_blocks.iter().map(|block| {
                    let on_select = {
                        let on_block_select = props.on_block_select.clone();
                        let block = block.clone();
                        Callback::from(move |_| {
                            on_block_select.emit(block.clone());
                        })
                    };
                    
                    html! {
                        <div 
                            class="block-browser-item" 
                            onclick={on_select}
                        >
                            <div class="block-browser-item-header">
                                <h3 class="block-browser-item-title">{&block.title}</h3>
                                <span class="block-browser-item-version">{&block.version}</span>
                            </div>
                            <p class="block-browser-item-description">{&block.description}</p>
                            <div class="block-browser-item-tags">
                                { for block.tags.iter().map(|tag| {
                                    html! {
                                        <span class="block-browser-tag">{tag}</span>
                                    }
                                })}
                            </div>
                            <div class="block-browser-item-meta">
                                <span class={format!("block-browser-purity block-browser-purity-{}", 
                                    match block.purity {
                                        shtairir_registry::model::Purity::Pure => "pure",
                                        shtairir_registry::model::Purity::Effect => "effect",
                                    }
                                )}>
                                    {match block.purity {
                                        shtairir_registry::model::Purity::Pure => "Pure",
                                        shtairir_registry::model::Purity::Effect => "Effect",
                                    }}
                                </span>
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}