use crate::domain::models::Document;
use crate::domain::errors::DocumentError;
use crate::infrastructure::pdf_exporter::PdfExporter;
use crate::infrastructure::docx_exporter::DocxExporter;
use uuid::Uuid;
use std::sync::Arc;

pub struct ExportService {
    pdf_exporter: Arc<PdfExporter>,
    docx_exporter: Arc<DocxExporter>,
}

impl ExportService {
    pub fn new(
        pdf_exporter: Arc<PdfExporter>,
        docx_exporter: Arc<DocxExporter>,
    ) -> Self {
        ExportService {
            pdf_exporter,
            docx_exporter,
        }
    }
    
    pub async fn export_document(
        &self,
        document: &Document,
        format: ExportFormat,
    ) -> Result<Vec<u8>, DocumentError> {
        match format {
            ExportFormat::Pdf => {
                self.pdf_exporter.export_document(document).await
            }
            ExportFormat::Docx => {
                self.docx_exporter.export_document(document).await
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExportFormat {
    Pdf,
    Docx,
}

impl ExportFormat {
    pub fn from_str(format: &str) -> Result<Self, DocumentError> {
        match format.to_lowercase().as_str() {
            "pdf" => Ok(ExportFormat::Pdf),
            "docx" => Ok(ExportFormat::Docx),
            _ => Err(DocumentError::ExportFailed(format!("Unsupported format: {}", format))),
        }
    }
}