# Invoice PDF Generation Architecture

This document outlines the architectural design for generating PDF invoices within the CPC platform. The design leverages existing backend logic and adheres to our established architectural principles.

## 1. Overview

The goal is to enable users to download a PDF version of any invoice. The system will reuse the existing, but currently disconnected, PDF generation capabilities in the `backend` service. The core of this work involves creating the necessary GraphQL and Tauri plumbing to connect the frontend UI to this existing functionality.

## 2. System Diagram

```mermaid
graph TD
    subgraph cpc-platform (Desktop App)
        A[Yew UI: Invoice Details Page] -->|1. User clicks "Download PDF"| B(Tauri Command: generate_invoice_pdf);
        B -->|2. Invokes GraphQL Mutation| C{GraphQL Client};
    end

    subgraph backend (Axum Server)
        C -->|3. Sends GenerateInvoicePdfMutation| D{GraphQL API};
        D -->|4. Resolves Mutation| E[Invoicing Service];
        E -->|5. Fetches Invoice, Customer, and Line Items| F[Database];
        E -->|6. Calls Existing PDF Generator| G(pdf.rs: generate_invoice_pdf);
        G -->|7. Returns PDF bytes| E;
        E -->|8. Returns Base64 encoded PDF| D;
    end

    C -->|9. Receives Base64 PDF| B;
    B -->|10. Decodes and prompts user to save file| H[User's File System];

```

## 3. Component Responsibilities

### `backend` Service

*   **GraphQL Mutation (`invoicing.rs`):**
    *   Define a new `GenerateInvoicePdf` mutation.
    *   This mutation will accept an `invoiceId`.
    *   It will fetch the full `Invoice`, its associated `Customer`, and its `InvoiceLineItem`s from the database.
    *   It will call the existing `invoicing::pdf::generate_invoice_pdf` function.
    *   It will return the resulting `Vec<u8>` as a Base64-encoded string.
*   **GraphQL `Invoice` Resolver (`models.rs`):**
    *   Implement the `line_items` resolver for the `Invoice` `ComplexObject`. This is a prerequisite for the mutation. It will query the `invoice_line_items` table based on the `invoice_id`.
*   **PDF Generation (`pdf.rs`):**
    *   No changes are required. The existing `generate_invoice_pdf` function will be used as-is.

### `cpc-platform` (Tauri App)

*   **Tauri Command (`invoicing/commands.rs`):**
    *   Create a new Tauri command: `generate_invoice_pdf(invoice_id: String)`.
    *   This command will be a thin wrapper that calls the `GenerateInvoicePdf` GraphQL mutation on the `backend`.
    *   Upon receiving the Base64 PDF string, it will decode it into bytes.
    *   It will use Tauri's `dialog` API to open a "Save File" dialog for the user.
    *   It will write the PDF bytes to the location selected by the user.
*   **Yew UI (`invoicing/components`):**
    *   A "Download PDF" button will be added to the invoice details view.
    *   Clicking this button will invoke the `generate_invoice_pdf` Tauri command with the current invoice's ID.

## 4. GraphQL Schema Proposal

A new mutation will be added to the schema.

```graphql
# backend/src/graphql/schema.graphql

type Mutation {
  # ... existing mutations
  generateInvoicePdf(invoiceId: ID!): String!
}
```

The result is a `String!`, which will contain the Base64-encoded PDF data.

## 5. Rust Type Definitions

No new PDF-specific option types are needed immediately, as the generation is handled on the backend. The primary type definition is for the new Tauri command.

```rust
// apps/cpc-platform/src-tauri/src/invoicing/commands.rs

#[tauri::command]
pub async fn generate_invoice_pdf(invoice_id: String) -> Result<(), String> {
    // 1. Call the backend's GraphQL `generateInvoicePdf` mutation.
    // 2. Receive the Base64 string response.
    // 3. Decode the Base64 string to a Vec<u8>.
    // 4. Use tauri::api::dialog::FileDialogBuilder to prompt the user to save the file.
    // 5. Write the bytes to the selected file path.
    // 6. Return Ok(()) or an error string.
    unimplemented!()
}
```

## 6. PDF Template Structure

The PDF structure is already defined in `apps/backend/src/invoicing/pdf.rs`. It includes:

*   **Header:** Invoice Number, Issue Date, Due Date.
*   **Bill To:** Customer Name, Email, and Address.
*   **Line Items Table:**
    *   Description
    *   Quantity
    *   Unit Price
    *   Tax Rate
    *   Total
*   **Totals Section:** Subtotal, Tax, and Grand Total.
*   **Footer:** Payment terms and notes (if available).

## 7. Security Considerations

*   **Authorization:** The `GenerateInvoicePdf` GraphQL mutation must be protected. It should verify that the authenticated user has the necessary permissions to access the requested `invoiceId`. This will be handled by the existing authentication and authorization logic in our GraphQL layer.
*   **Data Validation:** The `invoiceId` must be validated to prevent injection attacks, although the use of typed IDs in GraphQL provides a good layer of protection.
*   **Resource Usage:** PDF generation can be memory-intensive. While `genpdf` is efficient, we should monitor performance. If we see issues, we may need to offload PDF generation to a dedicated `cpc-node` worker in the future, but for now, the `backend` service is sufficient.

## 8. Implementation Plan

1.  **Backend:**
    *   Implement the `Invoice::line_items` resolver in `apps/backend/src/invoicing/models.rs`.
    *   Implement the `generateInvoicePdf` mutation in `apps/backend/src/graphql/invoicing.rs`.
2.  **Frontend:**
    *   Implement the `generate_invoice_pdf` Tauri command in `apps/cpc-platform/src-tauri/src/invoicing/commands.rs`.
    *   Add a "Download PDF" button to the relevant Yew component and wire it to the new Tauri command.