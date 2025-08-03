//! Image exporter for visualization data

use web_sys::HtmlCanvasElement;
use wasm_bindgen::JsCast;

/// Export canvas as image file
pub fn export_as_image(canvas: &HtmlCanvasElement, filename: &str) {
    // Get data URL from canvas
    let data_url = canvas.to_data_url().unwrap();
    
    // Create download link
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let link = document.create_element("a").unwrap();
    link.set_attribute("href", &data_url).unwrap();
    link.set_attribute("download", filename).unwrap();
    
    // Trigger download
    link.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
}

/// Export canvas as image with custom format and quality
pub fn export_as_image_with_options(
    canvas: &HtmlCanvasElement,
    filename: &str,
    format: &str,
    quality: Option<f64>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get data URL from canvas with options
    let data_url = if let Some(quality) = quality {
        canvas.to_data_url_with_type_and_quality(format, quality)?
    } else {
        canvas.to_data_url_with_type(format)?
    };
    
    // Create download link
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let link = document.create_element("a").unwrap();
    link.set_attribute("href", &data_url).unwrap();
    link.set_attribute("download", filename).unwrap();
    
    // Trigger download
    link.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
    
    Ok(())
}

/// Get image data as base64 string
pub fn get_image_data_base64(canvas: &HtmlCanvasElement) -> Result<String, Box<dyn std::error::Error>> {
    let data_url = canvas.to_data_url()?;
    // Remove data URL prefix
    let base64_data = data_url.split(',').nth(1).unwrap_or("").to_string();
    Ok(base64_data)
}