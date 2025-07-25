use crate::models::{Proposal, FeedItem, SupplyChain, SupplyChainNode, TransportationSegment, NodeType, TransportMethod};
use crate::product::model::Product;
use crate::types::{User, Comment, Post};
use cpc_protos::android_data::*;
use prost::Message;

impl From<&User> for AndroidUser {
    fn from(user: &User) -> Self {
        AndroidUser {
            id: user.id.clone(),
            name: user.name.clone(),
            email: user.email.clone(),
        }
    }
}

impl From<&Comment> for AndroidComment {
    fn from(comment: &Comment) -> Self {
        AndroidComment {
            id: comment.id.clone(),
            post_id: comment.post_id.clone(),
            author_id: comment.author_id.clone(),
            content: comment.content.clone(),
        }
    }
}

impl From<&Post> for AndroidPost {
    fn from(post: &Post) -> Self {
        AndroidPost {
            id: post.id.clone(),
            content: post.content.clone(),
            author_id: post.author_id.clone(),
            likes: post.likes,
            comments: post.comments.iter().map(|c| c.into()).collect(),
        }
    }
}

impl From<&Proposal> for AndroidProposal {
    fn from(proposal: &Proposal) -> Self {
        AndroidProposal {
            id: proposal.id.clone(),
            title: proposal.title.clone(),
            description: proposal.description.clone(),
            votes_for: proposal.votes_for,
            votes_against: proposal.votes_against,
        }
    }
}

impl From<&FeedItem> for AndroidFeedItem {
    fn from(item: &FeedItem) -> Self {
        match item {
            FeedItem::Post { id, content, author_id, likes, comments } => {
                AndroidFeedItem {
                    item: Some(android_feed_item::Item::Post(AndroidPost {
                        id: id.clone(),
                        content: content.clone(),
                        author_id: author_id.clone(),
                        likes: *likes,
                        comments: *comments,
                    }))
                }
            }
            FeedItem::Proposal(proposal) => {
                AndroidFeedItem {
                    item: Some(android_feed_item::Item::Proposal(proposal.into()))
                }
            }
        }
    }
}

impl From<&SupplyChain> for AndroidSupplyChain {
    fn from(sc: &SupplyChain) -> Self {
        AndroidSupplyChain {
            nodes: sc.nodes.iter().map(|n| n.into()).collect(),
            segments: sc.segments.iter().map(|s| s.into()).collect(),
        }
    }
}

impl From<&SupplyChainNode> for AndroidSupplyChainNode {
    fn from(node: &SupplyChainNode) -> Self {
        AndroidSupplyChainNode {
            id: node.id.clone(),
            node_type: match node.node_type {
                NodeType::RawMaterial => 0,
                NodeType::Manufacturer => 1,
                NodeType::Distributor => 2,
                NodeType::Retailer => 3,
            },
            location: node.location.clone(),
            company: node.company.clone(),
            timestamp: node.timestamp.clone(),
            coordinates: Some(AndroidCoordinates {
                latitude: node.coordinates.latitude,
                longitude: node.coordinates.longitude,
            }),
        }
    }
}

impl From<&TransportationSegment> for AndroidTransportationSegment {
    fn from(segment: &TransportationSegment) -> Self {
        AndroidTransportationSegment {
            from_node_id: segment.from_node_id.clone(),
            to_node_id: segment.to_node_id.clone(),
            method: match segment.method {
                TransportMethod::Ship => 0,
                TransportMethod::Truck => 1,
                TransportMethod::Plane => 2,
                TransportMethod::Train => 3,
            },
            duration_hours: segment.duration_hours,
            carbon_footprint: segment.carbon_footprint,
        }
    }
}

impl From<&Product> for AndroidProduct {
    fn from(product: &Product) -> Self {
        AndroidProduct {
            id: product.id.clone(),
            name: product.name.clone(),
            brand: product.brand.clone(),
            description: product.description.clone().unwrap_or_default(),
            barcode: product.barcode.clone(),
            carbon_footprint: product.carbon_footprint.unwrap_or(0.0),
            packaging_type: product.packaging_type.clone(),
            nutritional_info: product.nutritional_info.clone(),
            manufacturer: product.manufacturer.clone(),
            material_cost: product.material_cost.unwrap_or(0.0),
            labor_cost: product.labor_cost.unwrap_or(0.0),
            supplier: product.supplier.clone(),
            current_stock: product.current_stock.unwrap_or(0),
            reorder_level: product.reorder_level.unwrap_or(0),
            supply_chain: product.supply_chain.as_ref().map(|sc| sc.into()),
            cost: product.cost.as_ref().map(|money| AndroidMoney {
                amount: money.amount,
                currency: money.currency.clone(),
            }),
            location: product.location.as_ref().map(|loc| AndroidWarehouseLocation {
                id: loc.id.clone(),
                name: loc.name.clone(),
            }),
        }
    }
}

impl TryFrom<&AndroidComment> for Comment {
    type Error = anyhow::Error;

    fn try_from(proto: &AndroidComment) -> Result<Self, Self::Error> {
        Ok(Comment {
            id: proto.id.clone(),
            post_id: proto.post_id.clone(),
            author_id: proto.author_id.clone(),
            content: proto.content.clone(),
        })
    }
}

pub fn from_protobuf<T: Message + Default>(bytes: &[u8]) -> Result<T, anyhow::Error> {
    T::decode(bytes).map_err(|e| anyhow::anyhow!("Protobuf deserialization failed: {}", e))
}

pub fn to_protobuf<T: Message>(value: &T) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(value.encoded_len());
    value.encode(&mut buf).expect("Failed to encode protobuf");
    buf
}

// Timestamp conversion helpers
pub fn proto_timestamp_to_chrono(ts: &prost_types::Timestamp) -> chrono::NaiveDateTime {
    chrono::NaiveDateTime::from_timestamp_opt(ts.seconds, ts.nanos as u32)
        .unwrap_or_else(|| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap())
}

pub fn chrono_to_proto_timestamp(dt: &chrono::NaiveDateTime) -> prost_types::Timestamp {
    prost_types::Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos() as i32,
    }
}