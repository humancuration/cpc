// Supply chain visualization paused for future development
// use yew::prelude::*;
// use crate::types::SupplyChain;

// #[derive(Properties, PartialEq)]
// pub struct SupplyChainMapProps {
//     pub supply_chain: SupplyChain,
// }

// #[function_component(SupplyChainMap)]
// pub fn supply_chain_map(props: &SupplyChainMapProps) -> Html {
//     html! {
//         <div class="supply-chain-map">
//             <h3>{"Product Journey"}</h3>
//             <div class="map-container">
//                 // TODO: Implement Leaflet map visualization
//                 <div class="map-placeholder">{"Map visualization will go here"}</div>
//             </div>
//             <div class="timeline">
//                 <h4>{"Journey Timeline"}</h4>
//                 <ul>
//                     {for props.supply_chain.nodes.iter().map(|node| {
//                         html! {
//                             <li key={node.id.clone()}>
//                                 <strong>{format!("{:?}", node.node_type)}</strong>{" at "}
//                                 <span>{node.location.clone()}</span>{" ("}
//                                 <span>{node.company.clone()}</span>{") - "}
//                                 <time>{node.timestamp.format("%Y-%m-%d").to_string()}</time>
//                             </li>
//                         }
//                     })}
//                 </ul>
//             </div>
//         </div>
//     }
// }