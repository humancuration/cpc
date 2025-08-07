use yew::prelude::*;
use crate::models::Node;
use crate::registry::RegistryManager;

#[derive(Properties, PartialEq)]
pub struct BlockLibraryProps {
    pub registry: RegistryManager,
    pub on_add_node: Callback<String>,
}

#[function_component(BlockLibrary)]
pub fn block_library(props: &BlockLibraryProps) -> Html {
    let search_term = use_state(|| String::new());
    let selected_category = use_state(|| String::new());
    
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
    
    let categories = props.registry.get_categories();
    
    html! {
        <div class="block-library">
            <div class="library-header">
                <h2>{"Block Library"}</h2>
                <input
                    type="text"
                    placeholder="Search blocks..."
                    value={(*search_term).clone()}
                    oninput={on_search_input}
                    class="search-input"
                />
                <select onchange={on_category_change} class="category-select">
                    <option value="">{"All Categories"}</option>
                    { for categories.iter().map(|category| {
                        html! {
                            <option value={category.clone()}>{category}</option>
                        }
                    })}
                </select>
            </div>
            
            <div class="blocks-container">
                { for props.registry.get_blocks().iter().map(|block_spec| {
                    let on_add_click = {
                        let on_add_node = props.on_add_node.clone();
                        let block_id = format!("{}@{}:{}", block_spec.namespace, block_spec.version, block_spec.name);
                        Callback::from(move |_| {
                            on_add_node.emit(block_id.clone());
                        })
                    };
                    
                    html! {
                        <div class="block-item" onclick={on_add_click}>
                            <div class="block-header">
                                <h3>{&block_spec.title}</h3>
                                <span class="block-version">{&block_spec.version}</span>
                            </div>
                            <p class="block-description">{&block_spec.description}</p>
                            <div class="block-tags">
                                { for block_spec.tags.iter().map(|tag| {
                                    html! {
                                        <span class="tag">{tag}</span>
                                    }
                                })}
                            </div>
                            <div class="block-meta">
                                <span class={format!("purity-{}", 
                                    match block_spec.purity {
                                        shtairir_registry::model::Purity::Pure => "pure",
                                        shtairir_registry::model::Purity::Effect => "effect",
                                    }
                                )}>
                                    {match block_spec.purity {
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