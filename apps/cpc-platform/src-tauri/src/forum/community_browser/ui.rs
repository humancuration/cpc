use crate::store::forum::{Community, ForumStore};
use gloo_console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use crate::router::AppRoute;
use crate::store::forum::{Community, ForumStore};
use gloo_console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(CommunityBrowser)]
pub fn community_browser() -> Html {
    let (store, dispatch) = use_store::<ForumStore>();
    let search_term = use_state(|| "".to_string());
    let communities_loading = use_state(|| true);

    // --- MOCK DATA ---
    // In a real scenario, this effect would fetch data from GraphQL.
    use_effect_with((), move |_| {
        let dispatch = dispatch.clone();
        gloo_timers::callback::Timeout::new(1_500, move || {
            let mock_communities = vec![
                Community {
                    id: 1,
                    name: "FarmingCoop".to_string(),
                    slug: "farming-coop".to_string(),
                    about: "A place for farmers to collaborate.".to_string(),
                    created_at: chrono::Utc::now(),
                    member_count: 3200,
                    avatar: "https://i.pravatar.cc/150?img=1".parse().unwrap(),
                    is_joined: true,
                },
                Community {
                    id: 2,
                    name: "LocalMakers".to_string(),
                    slug: "local-makers".to_string(),
                    about: "DIY and local manufacturing enthusiasts.".to_string(),
                    created_at: chrono::Utc::now(),
                    member_count: 1100,
                    avatar: "https://i.pravatar.cc/150?img=2".parse().unwrap(),
                    is_joined: false,
                },
                Community {
                    id: 3,
                    name: "AltEnergy".to_string(),
                    slug: "alt-energy".to_string(),
                    about: "Discussing alternative energy solutions.".to_string(),
                    created_at: chrono::Utc::now(),
                    member_count: 812,
                    avatar: "https://i.pravatar.cc/150?img=3".parse().unwrap(),
                    is_joined: false,
                },
            ];

            dispatch.reduce_mut(|store| {
                store.communities = mock_communities;
            });
            communities_loading.set(false);
            log!("Mock data loaded.");
        })
        .forget();
    });
    // --- END MOCK DATA ---

    let on_search_input = {
        let search_term = search_term.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            search_term.set(input.value());
        })
    };

    let on_join_toggle = {
        let dispatch = dispatch.clone();
        Callback::from(move |id: u64| {
            dispatch.reduce_mut(|store| {
                if let Some(community) = store.communities.iter_mut().find(|c| c.id == id) {
                    community.is_joined = !community.is_joined;
                    log!(format!("Toggled join for community {}", id));
                }
            });
        })
    };

    let filtered_communities = store
        .communities
        .iter()
        .filter(|c| {
            c.name
                .to_lowercase()
                .contains(&search_term.to_lowercase())
        })
        .collect::<Vec<_>>();

    html! {
        <div class="container mx-auto p-4 font-sans">
            <div class="flex justify-between items-center mb-4">
                <div class="relative flex-grow">
                    <span class="absolute inset-y-0 left-0 flex items-center pl-3">
                        <svg class="w-5 h-5 text-gray-400" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                            <path d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
                        </svg>
                    </span>
                    <input
                        type="search"
                        placeholder="Search communities..."
                        class="w-full pl-10 pr-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                        oninput={on_search_input}
                        value={(*search_term).clone()}
                    />
                </div>
                <button
                    class="ml-4 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50"
                    // disabled={!user_has_create_permission}
                >
                    { "+ Create" }
                </button>
            </div>

            if *communities_loading {
                <CommunityListSkeleton />
            } else {
                <div class="space-y-2">
                {
                    for filtered_communities.iter().map(|community| {
                        html!{
                            <CommunityCard
                                community={(*community).clone()}
                                on_join_toggle={on_join_toggle.clone()}
                            />
                        }
                    })
                }
                </div>
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct CommunityCardProps {
    community: Community,
    on_join_toggle: Callback<u64>,
}

#[function_component(CommunityCard)]
fn community_card(props: &CommunityCardProps) -> Html {
    let navigator = use_navigator().unwrap();

    let on_card_click = {
        let slug = props.community.slug.clone();
        Callback::from(move |_| {
            navigator.push(&AppRoute::Community { slug: slug.clone() });
        })
    };

    let on_join_click = {
        let on_join_toggle = props.on_join_toggle.clone();
        let id = props.community.id;
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation(); // Prevent card click when clicking the button
            on_join_toggle.emit(id);
        })
    };

    let join_button_classes = if props.community.is_joined {
        "px-4 py-1 text-sm border border-gray-400 text-gray-600 rounded-full hover:bg-gray-100"
    } else {
        "px-4 py-1 text-sm bg-blue-500 text-white rounded-full hover:bg-blue-600"
    };

    html! {
        <div onclick={on_card_click} class="flex items-center p-3 border rounded-lg hover:bg-gray-50 cursor-pointer transition-colors">
            <img src={props.community.avatar.to_string()} alt="avatar" class="w-12 h-12 rounded-full mr-4" />
            <div class="flex-grow">
                <h3 class="font-bold text-lg">{ &props.community.name }</h3>
                <p class="text-sm text-gray-500">{ format!("{} members", props.community.member_count) }</p>
            </div>
            <button onclick={on_join_click} class={join_button_classes}>
                { if props.community.is_joined { "Joined ✓" } else { "Join" } }
            </button>
        </div>
    }
}

#[function_component(CommunityListSkeleton)]
fn community_list_skeleton() -> Html {
    html! {
        <div class="space-y-2">
            { for (0..3).map(|_| html! {
                <div class="flex items-center p-3 border rounded-lg animate-pulse">
                    <div class="w-12 h-12 rounded-full bg-gray-300 mr-4"></div>
                    <div class="flex-grow space-y-2">
                        <div class="h-4 bg-gray-300 rounded w-1/3"></div>
                        <div class="h-3 bg-gray-300 rounded w-1/4"></div>
                    </div>
                    <div class="w-20 h-8 bg-gray-300 rounded-full"></div>
                </div>
            })}
        </div>
    }
}