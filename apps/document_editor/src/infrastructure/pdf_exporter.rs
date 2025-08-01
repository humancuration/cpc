use crate::domain::models::Document;
use crate::domain::errors::DocumentError;
use crate::crdt::operations::{DocumentOperation, FormatType, FormattingStyle};
use pdf::*;
use uuid::Uuid;
use std::collections::HashMap;

pub struct PdfExporter;

impl PdfExporter {
    pub fn new() -> Self {
        PdfExporter
    }
    
    pub fn to_pdf(operations: Vec<DocumentOperation>, images: HashMap<Uuid, Vec<u8>>) -> Result<Vec<u8>, DocumentError> {
        // Replay operations to build formatted content
        let mut content = String::new();
        let mut formatting = Vec::new();
        let mut images_data = Vec::new();
        
        for op in operations {
            match op {
                DocumentOperation::Format { range, format } => {
                    formatting.push((range, format));
                },
                DocumentOperation::Formatting { style, .. } => {
                    // Handle the older Formatting operation type
                    // Convert to Format operation for consistency
                    match style {
                        FormattingStyle::Bold => {
                            formatting.push(((
                                collaboration_engine::core::Position { line: 0, column: 0 },
                                collaboration_engine::core::Position { line: 0, column: 0 }
                            ), FormatType::Bold));
                        },
                        FormattingStyle::Italic => {
                            formatting.push(((
                                collaboration_engine::core::Position { line: 0, column: 0 },
                                collaboration_engine::core::Position { line: 0, column: 0 }
                            ), FormatType::Italic));
                        },
                        FormattingStyle::Underline => {
                            formatting.push(((
                                collaboration_engine::core::Position { line: 0, column: 0 },
                                collaboration_engine::core::Position { line: 0, column: 0 }
                            ), FormatType::Underline));
                        },
                        FormattingStyle::Heading(level) => {
                            // Treat headings as bold for now
                            formatting.push(((
                                collaboration_engine::core::Position { line: 0, column: 0 },
                                collaboration_engine::core::Position { line: 0, column: 0 }
                            ), FormatType::Bold));
                        },
                    }
                },
                DocumentOperation::Insert { value, .. } => {
                    // For simplicity, we're just extracting the text value
                    if let Ok(text) = serde_json::from_value::<String>(value) {
                        content.push_str(&text);
                    }
                },
                DocumentOperation::InsertImage { position, image_id, caption } => {
                    // Handle image insertion
                    if let Some(image_data) = images.get(&image_id) {
                        images_data.push((position, image_data.clone(), caption));
                    }
                },
                // Handle other operations as needed...
                _ => {}
            }
        }
        
        // Create PDF document
        let mut doc = pdf::Document::new();
        
        // Add a simple page with the document content
        let page = doc.add_page(595.0, 842.0, "Page 1");
        
        // Add content with basic formatting
        let font = doc.add_builtin_font(pdf::BuiltinFont::Helvetica).unwrap();
        let bold_font = doc.add_builtin_font(pdf::BuiltinFont::HelveticaBold).unwrap();
        let italic_font = doc.add_builtin_font(pdf::BuiltinFont::HelveticaOblique).unwrap();
        
        let mut y_position = 750.0;
        
        // Split content into lines for better formatting
        let lines: Vec<&str> = content.lines().collect();
        
        for line in lines {
            // Check if this line has any formatting applied
            let mut current_font = font;
            let mut font_size = 12.0;
            
            // Apply formatting based on detected formats
            for (range, format) in &formatting {
                match format {
                    FormatType::Bold => {
                        current_font = bold_font;
                        font_size = 12.0;
                    },
                    FormatType::Italic => {
                        current_font = italic_font;
                        font_size = 12.0;
                    },
                    FormatType::Underline => {
                        // PDF crate doesn't directly support underline, but we can simulate it
                        current_font = font;
                        font_size = 12.0;
                    },
                    FormatType::ListItem => {
                        // Add bullet point for list items
                        let bullet_line = format!("• {}", line);
                        doc.add_text(
                            page,
                            50.0, y_position,
                            &bullet_line,
                            font,
                            font_size,
                        );
                        y_position -= 20.0;
                        break; // Skip adding the line again
                    },
                }
            }
            
            // Add the text with the appropriate formatting
            doc.add_text(
                page,
                50.0, y_position,
                line,
                current_font,
                font_size,
            );
            
            y_position -= 20.0;
            
            // Check if we need to add a new page
            if y_position < 50.0 {
                let page = doc.add_page(595.0, 842.0, "Page 2");
                y_position = 750.0;
            }
        }
        
        // Add images
        for (position, image_data, caption) in images_data {
            // In a real implementation, we would embed the image in the PDF
            // For now, we'll just add a placeholder text
            doc.add_text(
                page,
                50.0, y_position,
                &format!("[Image: {}]", caption),
                font,
                10.0,
            );
            y_position -= 30.0;
            
            // Check if we need to add a new page
            if y_position < 50.0 {
                let page = doc.add_page(595.0, 842.0, "Page 2");
                y_position = 750.0;
            }
        }
        
        // Save to bytes
        let mut buffer = Vec::new();
        doc.save_to(&mut buffer).map_err(|e| DocumentError::ExportFailed(e.to_string()))?;
        
        Ok(buffer)
    }
    
    pub async fn export_document(&self, document: &Document) -> Result<Vec<u8>, DocumentError> {
        // In a real implementation, this would:
        // 1. Convert the document content to PDF format
        // 2. Handle embedded images using the media processor
        // 3. Apply formatting and styling
        // 4. Return the PDF as bytes
        
        // For now, we'll create a simple PDF with the document title and a placeholder
        let mut doc = pdf::Document::new();
        
        // Add a simple page with the document title
        let page = doc.add_page(595.0, 842.0, "Page 1");
        let font = doc.add_builtin_font(pdf::BuiltinFont::Helvetica).unwrap();
        let bold_font = doc.add_builtin_font(pdf::BuiltinFont::HelveticaBold).unwrap();
        
        doc.add_text(
            page,
            50.0, 750.0,
            document.title.as_str(),
            bold_font,
            24.0,
        );
        
        doc.add_text(
            page,
            50.0, 700.0,
            "Document content would be here...",
            font,
            12.0,
        );
        
        // Save to bytes
        let mut buffer = Vec::new();
        doc.save_to(&mut buffer).map_err(|e| DocumentError::ExportFailed(e.to_string()))?;
        
        Ok(buffer)
    }
}