use yew::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Properties, PartialEq, Clone)]
pub struct SupplyChainDisplayProps {
    pub supply_chain: Option<SupplyChain>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupplyChain {
    pub nodes: Vec<SupplyChainNode>,
    pub certifications: Vec<Certification>,
    pub cooperative_metrics: CooperativeMetrics,
    pub transport_segments: Vec<TransportSegment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupplyChainNode {
    pub id: String,
    pub node_type: NodeType,
    pub name: String,
    pub location: String,
    pub company: String,
    pub timestamp: String,
    pub coordinates: Coordinates,
    pub certifications: Vec<String>,
    pub cooperative_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeType {
    RawMaterial,
    Manufacturer,
    Distributor,
    Retailer,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Certification {
    pub id: String,
    pub name: String,
    pub issuer: String,
    pub issued_date: String,
    pub valid_until: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CooperativeMetrics {
    pub total_cooperatives: u32,
    pub cooperative_revenue_share: f64,
    pub fair_trade_percentage: f64,
    pub worker_owned_percentage: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportSegment {
    pub from_node_id: String,
    pub to_node_id: String,
    pub method: TransportMethod,
    pub distance_km: f64,
    pub duration_hours: u32,
    pub carbon_footprint: f64,
    pub cost: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransportMethod {
    Ship,
    Truck,
    Plane,
    Train,
}

#[function_component(SupplyChainDisplay)]
pub fn supply_chain_display(props: &SupplyChainDisplayProps) -> Html {
    let product = &props.product;
    
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
            let total_carbon = supply_chain.transport_segments.iter()
                .map(|s| s.carbon_footprint)
                .sum::<f64>();
            
            let total_cost = supply_chain.transport_segments.iter()
                .map(|s| s.cost)
                .sum::<f64>();
            
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
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                                    </svg>
                                    { format!("Cost: ${:.2}", total_cost) }
                                </div>
                            </div>
                        </div>
                    </div>
                    
                    <div class="px-6 py-4">
                        // Cooperative Metrics
                        <div class="mb-6 p-4 bg-blue-50 rounded-lg">
                            <h3 class="text-sm font-medium text-blue-900 mb-2">{ "Cooperative Impact" }</h3>
                            <div class="grid grid-cols-2 gap-4 text-sm">
                                <div>
                                    <span class="text-blue-600">{ "Total Cooperatives: " }</span>
                                    <span class="font-medium">{ supply_chain.cooperative_metrics.total_cooperatives }</span>
                                </div>
                                <div>
                                    <span class="text-blue-600">{ "Revenue Share: " }</span>
                                    <span class="font-medium">{ format!("{:.1}%", supply_chain.cooperative_metrics.cooperative_revenue_share * 100.0) }</span>
                                </div>
                                <div>
                                    <span class="text-blue-600">{ "Fair Trade: " }</span>
                                    <span class="font-medium">{ format!("{:.1}%", supply_chain.cooperative_metrics.fair_trade_percentage * 100.0) }</span>
                                </div>
                                <div>
                                    <span class="text-blue-600">{ "Worker Owned: " }</span>
                                    <span class="font-medium">{ format!("{:.1}%", supply_chain.cooperative_metrics.worker_owned_percentage * 100.0) }</span>
                                </div>
                            </div>
                        </div>
                        
                        // Certifications
                        if !supply_chain.certifications.is_empty() {
                            <div class="mb-6">
                                <h3 class="text-sm font-medium text-gray-900 mb-2">{ "Certifications" }</h3>
                                <div class="flex flex-wrap gap-2">
                                    { for supply_chain.certifications.iter().map(|cert| html! {
                                        <span class="inline-flex items-center px-3 py-1 rounded-full text-xs font-medium bg-green-100 text-green-800">
                                            <span class="mr-1">{ get_certification_icon(&cert.name) }</span>
                                            { &cert.name }
                                        </span>
                                    })}
                                </div>
                            </div>
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
                                                <span class={`inline-flex items-center justify-center w-8 h-8 rounded-full ${get_node_type_color(&node.node_type)}`}>
                                                    <span class="text-sm">{ get_node_type_icon(&node.node_type) }</span>
                                                </span>
                                            </div>
                                            
                                            <div class="min-w-0 flex-1">
                                                <div>
                                                    <div class="text-sm">
                                                        <span class={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${get_node_type_color(&node.node_type)}`}>
                                                            { format!("{:?}", node.node_type) }
                                                        </span>
                                                        { node.cooperative_id.as_ref().map(|id| html! {
                                                            <span class="ml-2 inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium bg-purple-100 text-purple-800">
                                                                { format!("Co-op: {}", id) }
                                                            </span>
                                                        }) }
                                                    </div>
                                                    <div class="mt-1 text-sm font-medium text-gray-900">{ &node.name }</div>
                                                    <div class="mt-1 text-sm text-gray-500">{ &node.company }</div>
                                                    <div class="mt-1 text-sm text-gray-500">{ &node.location }</div>
                                                    <div class="mt-1 text-xs text-gray-400">
                                                        { format!("Lat: {:.4}, Lng: {:.4}", node.coordinates.latitude, node.coordinates.longitude) }
                                                    </div>
                                                    
                                                    // Node certifications
                                                    if !node.certifications.is_empty() {
                                                        <div class="mt-2">
                                                            <div class="flex flex-wrap gap-1">
                                                                { for node.certifications.iter().map(|cert_id| {
                                                                    let cert = supply_chain.certifications.iter()
                                                                        .find(|c| c.id == *cert_id)
                                                                        .unwrap();
                                                                    html! {
                                                                        <span class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800">
                                                                            { get_certification_icon(&cert.name) }
                                                                            { " " }
                                                                            { &cert.name }
                                                                        </span>
                                                                    }
                                                                })}
                                                            </div>
                                                        </div>
                                                    }
                                                </div>
                                                
                                                {!is_last.then(|| {
                                                    let segment = supply_chain.transport_segments.iter()
                                                        .find(|s| s.from_node_id == node.id)
                                                        .unwrap();
                                                    
                                                    html! {
                                                        <div class="mt-4 flex items-center space-x-2 text-sm text-gray-500">
                                                            <span>{ get_transport_icon(&segment.method) }</span>
                                                            <span>{ format!("{:?} • {:.1}km • {}h • {:.2} kg CO₂ • ${:.2}",
                                                                segment.method,
                                                                segment.distance_km,
                                                                segment.duration_hours,
                                                                segment.carbon_footprint,
                                                                segment.cost) }</span>
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