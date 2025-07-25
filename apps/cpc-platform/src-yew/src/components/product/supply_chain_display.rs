use cpc_core::supply_chain::models::*;
use yew::prelude::*;
use crate::components::product::SupplyChainCharts;

#[derive(Properties, PartialEq, Clone)]
pub struct SupplyChainDisplayProps {
    pub supply_chain: Option<SupplyChain>,
}

#[function_component(SupplyChainDisplay)]
pub fn supply_chain_display(props: &SupplyChainDisplayProps) -> Html {
    let get_node_type_icon = |node_type: &NodeType| -> &'static str {
        match node_type {
            NodeType::RawMaterial => "🌱",
            NodeType::Manufacturer => "🏭",
            NodeType::Distributor => "🚛",
            NodeType::Retailer => "🏪",
        }
    };

    let get_transport_icon = |method: &TransportMethod| -> &'static str {
        match method {
            TransportMethod::Ship => "🚢",
            TransportMethod::Truck => "🚛",
            TransportMethod::Plane => "✈️",
            TransportMethod::Train => "🚂",
        }
    };

    let get_node_type_color = |node_type: &NodeType| -> &'static str {
        match node_type {
            NodeType::RawMaterial => "bg-green-100 text-green-800",
            NodeType::Manufacturer => "bg-blue-100 text-blue-800",
            NodeType::Distributor => "bg-yellow-100 text-yellow-800",
            NodeType::Retailer => "bg-purple-100 text-purple-800",
        }
    };

    let get_certification_icon = |cert_name: &str| -> &'static str {
        match cert_name.to_lowercase().as_str() {
            name if name.contains("organic") => "🌿",
            name if name.contains("fair") => "⚖️",
            name if name.contains("carbon") => "🌱",
            _ => "🏅",
        }
    };

    match &props.supply_chain {
        None => html! {
            <div class="bg-white shadow rounded-lg">
                <div class="px-6 py-4 border-b border-gray-200">
                    <h2 class="text-lg font-medium text-gray-900">{ "Supply Chain" }</h2>
                </div>
                <div class="px-6 py-12">
                    <div class="text-center">
                        <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
                        </svg>
                        <h3 class="mt-2 text-sm font-medium text-gray-900">{ "No supply chain data" }</h3>
                        <p class="mt-1 text-sm text-gray-500">{ "Supply chain information is not available for this product." }</p>
                    </div>
                </div>
            </div>
        },
        Some(supply_chain) => {
            let total_carbon = supply_chain.segments.iter()
                .map(|s| s.environmental_impact.carbon_footprint_kg_co2)
                .sum::<f64>();
            
            let cooperative_nodes = supply_chain.nodes.iter()
                .filter(|node| node.cooperative_metrics.is_some())
                .count();
            
            html! {
                <div class="bg-white shadow rounded-lg">
                    <div class="px-6 py-4 border-b border-gray-200">
                        <div class="flex items-center justify-between">
                            <h2 class="text-lg font-medium text-gray-900">{ "Supply Chain" }</h2>
                            <div class="flex items-center space-x-4 text-sm text-gray-500">
                                <div class="flex items-center">
                                    <svg class="h-5 w-5 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
                                    </svg>
                                    { format!("Carbon: {:.2} kg CO₂", total_carbon) }
                                </div>
                                <div class="flex items-center">
                                    <svg class="h-5 w-5 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                                    </svg>
                                    { format!("Co-ops: {}", cooperative_nodes) }
                                </div>
                            </div>
                        </div>
                    </div>
                    
                    <div class="px-6 py-4">
                        // Business Intelligence Charts
                        <SupplyChainCharts supply_chain={supply_chain.clone()} />
                        
                        // Cooperative Metrics
                        if cooperative_nodes > 0 {
                            <div class="mb-6 p-4 bg-blue-50 rounded-lg">
                                <h3 class="text-sm font-medium text-blue-900 mb-2">{ "Cooperative Impact" }</h3>
                                <div class="grid grid-cols-2 gap-4 text-sm">
                                    <div>
                                        <span class="text-blue-600">{ "Total Nodes: " }</span>
                                        <span class="font-medium">{ supply_chain.nodes.len() }</span>
                                    </div>
                                    <div>
                                        <span class="text-blue-600">{ "Co-op Nodes: " }</span>
                                        <span class="font-medium">{ cooperative_nodes }</span>
                                    </div>
                                    <div>
                                        <span class="text-blue-600">{ "Co-op Share: " }</span>
                                        <span class="font-medium">{ format!("{:.1}%", (cooperative_nodes as f64 / supply_chain.nodes.len() as f64) * 100.0) }</span>
                                    </div>
                                    <div>
                                        <span class="text-blue-600">{ "Total Carbon: " }</span>
                                        <span class="font-medium">{ format!("{:.2} kg", total_carbon) }</span>
                                    </div>
                                </div>
                            </div>
                        }
                        
                        // Certifications
                        {
                            let all_certifications: Vec<&EthicalCertification> = supply_chain.nodes
                                .iter()
                                .flat_map(|node| &node.certifications)
                                .collect();
                            
                            if !all_certifications.is_empty() {
                                <div class="mb-6">
                                    <h3 class="text-sm font-medium text-gray-900 mb-2">{ "Certifications" }</h3>
                                    <div class="flex flex-wrap gap-2">
                                        { for all_certifications.iter().map(|cert| html! {
                                            <span class="inline-flex items-center px-3 py-1 rounded-full text-xs font-medium bg-green-100 text-green-800">
                                                <span class="mr-1">{ get_certification_icon(&cert.name) }</span>
                                                { &cert.name }
                                            </span>
                                        })}
                                    </div>
                                </div>
                            }
                        }
                        
                        // Supply Chain Flow
                        <div class="space-y-6">
                            { for supply_chain.nodes.iter().enumerate().map(|(index, node)| {
                                let is_last = index == supply_chain.nodes.len() - 1;
                                
                                html! {
                                    <div class="relative">
                                        {!is_last.then(|| html! {
                                            <div class="absolute left-4 top-8 -ml-px mt-0.5 h-full w-0.5 bg-gray-300"></div>
                                        })}
                                        
                                        <div class="relative flex items-start space-x-3">
                                            <div>
                                                <span class={format!("inline-flex items-center justify-center w-8 h-8 rounded-full {}", get_node_type_color(&node.node_type))}>
                                                    <span class="text-sm">{ get_node_type_icon(&node.node_type) }</span>
                                                </span>
                                            </div>
                                            
                                            <div class="min-w-0 flex-1">
                                                <div>
                                                    <div class="text-sm">
                                                        <span class={format!("inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {}", get_node_type_color(&node.node_type))}>
                                                            { format!("{:?}", node.node_type) }
                                                        </span>
                                                        { node.cooperative_metrics.as_ref().map(|metrics| html! {
                                                            <span class="ml-2 inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium bg-purple-100 text-purple-800">
                                                                { format!("Co-op: {:.1}%", metrics.profit_sharing_percentage) }
                                                            </span>
                                                        }) }
                                                    </div>
                                                    <div class="mt-1 text-sm font-medium text-gray-900">{ &node.name }</div>
                                                    <div class="mt-1 text-sm text-gray-500">{ &node.location }</div>
                                                    <div class="mt-1 text-xs text-gray-400">
                                                        { format!("Lat: {:.4}, Lng: {:.4}", node.coordinates.0, node.coordinates.1) }
                                                    </div>
                                                    
                                                    // Node certifications
                                                    if !node.certifications.is_empty() {
                                                        <div class="mt-2">
                                                            <div class="flex flex-wrap gap-1">
                                                                { for node.certifications.iter().map(|cert| html! {
                                                                    <span class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800">
                                                                        { get_certification_icon(&cert.name) }
                                                                        { " " }
                                                                        { &cert.name }
                                                                    </span>
                                                                })}
                                                            </div>
                                                        </div>
                                                    }
                                                </div>
                                                
                                                {!is_last.then(|| {
                                                    let segment = supply_chain.segments.iter()
                                                        .find(|s| s.from_node_id == node.id)
                                                        .unwrap_or_else(|| {
                                                            // Find segment that starts from this node
                                                            supply_chain.segments.get(index).unwrap()
                                                        });
                                                    
                                                    html! {
                                                        <div class="mt-4 flex items-center space-x-2 text-sm text-gray-500">
                                                            <span>{ get_transport_icon(&segment.method) }</span>
                                                            <span>{ format!("{:?} • {}h • {:.2} kg CO₂",
                                                                segment.method,
                                                                segment.duration_hours,
                                                                segment.environmental_impact.carbon_footprint_kg_co2) }</span>
                                                        </div>
                                                    }
                                                })}
                                            </div>
                                        </div>
                                    </div>
                                }
                            })}
                        </div>
                    </div>
                </div>
            }
        }
    }
}