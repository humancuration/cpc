use yew::prelude::*;
use crate::types::Money;

#[derive(Properties, PartialEq)]
pub struct ProductPreviewProps {
    pub title: String,
    pub description: String,
    pub price: f64,
    pub currency: String,
    pub vendor_name: String,
    pub vendor_reputation: f64,
}

#[function_component(ProductPreview)]
pub fn product_preview(props: &ProductPreviewProps) -> Html {
    let formatted_price = format!("{:.2}", props.price);
    let reputation_stars = generate_stars(props.vendor_reputation);
    
    html! {
        <div class="product-preview">
            <div class="product-info">
                <h3 class="product-title">{ &props.title }</h3>
                <p class="product-description">{ &props.description }</p>
                
                <div class="price-section">
                    <span class="currency">{ &props.currency }</span>
                    <span class="price">{ formatted_price }</span>
                </div>
                
                <div class="vendor-info">
                    <span class="vendor-name">{ &props.vendor_name }</span>
                    <div class="vendor-reputation">
                        <span class="stars">{ reputation_stars }</span>
                        <span class="rating">{ format!("{:.1}", props.vendor_reputation) }</span>
                    </div>
                </div>
                
                <div class="tags">
                    <span class="tag">{ "Eco-friendly" }</span>
                    <span class="tag">{ "Fair Trade" }</span>
                </div>
            </div>
        </div>
    }
}

fn generate_stars(rating: f64) -> String {
    let full_stars = rating.floor() as usize;
    let has_half_star = rating - rating.floor() >= 0.5;
    
    let mut stars = String::new();
    
    for _ in 0..full_stars {
        stars.push('★');
    }
    
    if has_half_star {
        stars.push('☆');
    }
    
    while stars.len() < 5 {
        stars.push('☆');
    }
    
    stars
}