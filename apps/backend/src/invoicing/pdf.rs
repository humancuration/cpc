use genpdf::{elements, style, Element as _, Margins};
use anyhow::Result;
use crate::invoicing::models::{Invoice, Customer, InvoiceLineItem};

pub fn generate_invoice_pdf(invoice: &Invoice, customer: &Customer, line_items: &Vec<InvoiceLineItem>) -> Result<Vec<u8>> {
    // Create a new PDF document
    let font_family = genpdf::fonts::from_files("./fonts/liberation", "LiberationSans", None)?;
    let mut doc = genpdf::Document::new(font_family);
    doc.set_title("Invoice");

    // Add invoice header
    let mut header = elements::LinearLayout::vertical();
    header.push(
        elements::Text::new(format!("Invoice #{}", invoice.invoice_number))
            .with_style(style::Style::new().bold().with_font_size(24)),
    );
    header.push(elements::Text::new(format!("Issue Date: {}", invoice.issue_date)));
    header.push(elements::Text::new(format!("Due Date: {}", invoice.due_date)));
    doc.push(header);

    // Add customer details
    let mut customer_details = elements::LinearLayout::vertical();
    customer_details.push(elements::Text::new("Bill To:").with_style(style::Style::new().bold()));
    customer_details.push(elements::Text::new(&customer.name));
    if let Some(email) = &customer.email {
        customer_details.push(elements::Text::new(email));
    }
    if let Some(address) = &customer.address {
        customer_details.push(elements::Text::new(address));
    }
    doc.push(customer_details.with_margins(Margins::all(10.0)));

    // Create a table for the line items
    let mut table = elements::TableLayout::new(vec![1, 1, 1, 1, 1]);
    table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));

    // Add table header
    table.row()
        .push_element(elements::Text::new("Description").with_style(style::Style::new().bold()))
        .push_element(elements::Text::new("Quantity").with_style(style::Style::new().bold()))
        .push_element(elements::Text::new("Unit Price").with_style(style::Style::new().bold()))
        .push_element(elements::Text::new("Tax Rate").with_style(style::Style::new().bold()))
        .push_element(elements::Text::new("Total").with_style(style::Style::new().bold()))
        .collect::<()>();

    // Add line items
    for item in line_items {
        table.row()
            .push_element(elements::Text::new(&item.description))
            .push_element(elements::Text::new(item.quantity.to_string()))
            .push_element(elements::Text::new(item.unit_price.to_string()))
             .push_element(elements::Text::new(item.tax_rate.as_ref().map_or("N/A".to_string(), |r| format!("{:.2}%", r))))
            .push_element(elements::Text::new(item.total.to_string()))
            .collect::<()>();
    }
    doc.push(table);
    
    // Add totals
    let mut totals = elements::LinearLayout::vertical();
    totals.push(elements::Text::new(format!("Subtotal: {} {}", invoice.subtotal, invoice.currency)).with_alignment(genpdf::elements::Alignment::Right));
    totals.push(elements::Text::new(format!("Tax: {} {}", invoice.tax_total, invoice.currency)).with_alignment(genpdf::elements::Alignment::Right));
    totals.push(elements::Text::new(format!("Total: {} {}", invoice.total, invoice.currency)).with_alignment(genpdf::elements::Alignment::Right).with_style(style::Style::new().bold()));
    doc.push(totals.with_margins(Margins::all(10.0)));


    // Render the document to a byte vector
    let mut pdf_bytes = Vec::new();
    doc.render(&mut pdf_bytes)?;
    Ok(pdf_bytes)
}