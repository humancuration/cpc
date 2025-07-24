import { gql } from 'graphql-request';
import { client } from './client.js';

export const INVOICE_QUERIES = {
  GET_INVOICES: gql`
    query GetInvoices($filter: InvoiceFilter, $pagination: PaginationInput) {
      invoices(filter: $filter, pagination: $pagination) {
        id
        invoice_number
        recipient {
          id
          name
          email
        }
        issue_date
        due_date
        total_amount
        status
        items {
          id
          description
          quantity
          unit_price
          total
        }
        template {
          id
          name
        }
        created_at
        updated_at
      }
    }
  `,

  GET_INVOICE: gql`
    query GetInvoice($id: ID!) {
      invoice(id: $id) {
        id
        invoice_number
        recipient {
          id
          name
          email
          address
        }
        issue_date
        due_date
        total_amount
        status
        items {
          id
          description
          quantity
          unit_price
          total
        }
        template {
          id
          name
          header
          footer
          color_scheme
          font_family
        }
        notes
        tax_rate
        discount
        created_at
        updated_at
      }
    }
  `,

  GET_AGING_REPORT: gql`
    query GetAgingReport($dateRange: DateRangeInput!) {
      agingReport(dateRange: $dateRange) {
        invoices {
          id
          invoice_number
          customer_name
          issue_date
          due_date
          amount
          status
          days_overdue
        }
        summary {
          current
          overdue_1_30
          overdue_31_60
          overdue_61_90
          overdue_90_plus
          total
        }
      }
    }
  `,

  GET_SUPPLIER_PERFORMANCE: gql`
    query GetSupplierPerformance($dateRange: DateRangeInput!) {
      supplierPerformance(dateRange: $dateRange) {
        supplier_id
        supplier_name
        total_invoices
        total_amount
        average_payment_days
        on_time_rate
        last_payment_date
      }
    }
  `
};

export const INVOICE_MUTATIONS = {
  CREATE_INVOICE: gql`
    mutation CreateInvoice($input: CreateInvoiceInput!) {
      createInvoice(input: $input) {
        id
        invoice_number
        status
        total_amount
      }
    }
  `,

  UPDATE_INVOICE: gql`
    mutation UpdateInvoice($id: ID!, $input: UpdateInvoiceInput!) {
      updateInvoice(id: $id, input: $input) {
        id
        status
        total_amount
      }
    }
  `,

  DELETE_INVOICE: gql`
    mutation DeleteInvoice($id: ID!) {
      deleteInvoice(id: $id) {
        success
      }
    }
  `,

  SEND_INVOICE: gql`
    mutation SendInvoice($id: ID!) {
      sendInvoice(id: $id) {
        id
        status
        sent_at
      }
    }
  `,

  MARK_INVOICE_PAID: gql`
    mutation MarkInvoicePaid($id: ID!, $paid_at: DateTime!) {
      markInvoicePaid(id: $id, paid_at: $paid_at) {
        id
        status
        paid_at
      }
    }
  `
};

export const INVOICE_SUBSCRIPTIONS = {
  INVOICE_STATUS_CHANGED: gql`
    subscription InvoiceStatusChanged {
      invoiceStatusChanged {
        id
        status
        updated_at
      }
    }
  `
};

// Helper functions
export async function createInvoice(input) {
  try {
    const result = await client.request(INVOICE_MUTATIONS.CREATE_INVOICE, { input });
    return result.createInvoice;
  } catch (error) {
    console.error('Error creating invoice:', error);
    throw error;
  }
}

export async function getInvoices(filter = {}, pagination = {}) {
  try {
    const result = await client.request(INVOICE_QUERIES.GET_INVOICES, { filter, pagination });
    return result.invoices;
  } catch (error) {
    console.error('Error fetching invoices:', error);
    throw error;
  }
}

export async function getInvoice(id) {
  try {
    const result = await client.request(INVOICE_QUERIES.GET_INVOICE, { id });
    return result.invoice;
  } catch (error) {
    console.error('Error fetching invoice:', error);
    throw error;
  }
}

export async function updateInvoice(id, input) {
  try {
    const result = await client.request(INVOICE_MUTATIONS.UPDATE_INVOICE, { id, input });
    return result.updateInvoice;
  } catch (error) {
    console.error('Error updating invoice:', error);
    throw error;
  }
}

export async function deleteInvoice(id) {
  try {
    const result = await client.request(INVOICE_MUTATIONS.DELETE_INVOICE, { id });
    return result.deleteInvoice;
  } catch (error) {
    console.error('Error deleting invoice:', error);
    throw error;
  }
}

export async function sendInvoice(id) {
  try {
    const result = await client.request(INVOICE_MUTATIONS.SEND_INVOICE, { id });
    return result.sendInvoice;
  } catch (error) {
    console.error('Error sending invoice:', error);
    throw error;
  }
}

export async function markInvoicePaid(id, paid_at = new Date().toISOString()) {
  try {
    const result = await client.request(INVOICE_MUTATIONS.MARK_INVOICE_PAID, { id, paid_at });
    return result.markInvoicePaid;
  } catch (error) {
    console.error('Error marking invoice paid:', error);
    throw error;
  }
}

// Template queries and mutations
export const TEMPLATE_QUERIES = {
  GET_TEMPLATES: gql`
    query GetTemplates {
      templates {
        id
        name
        color_scheme
        font_family
        usage_count
        created_at
        is_default
      }
    }
  `,

  GET_TEMPLATE: gql`
    query GetTemplate($id: ID!) {
      template(id: $id) {
        id
        name
        header
        footer
        color_scheme
        font_family
        show_logo
        show_due_date
        show_payment_terms
        payment_terms
        created_at
        updated_at
      }
    }
  `
};

export const TEMPLATE_MUTATIONS = {
  CREATE_TEMPLATE: gql`
    mutation CreateTemplate($input: CreateTemplateInput!) {
      createTemplate(input: $input) {
        id
        name
      }
    }
  `,

  UPDATE_TEMPLATE: gql`
    mutation UpdateTemplate($id: ID!, $input: UpdateTemplateInput!) {
      updateTemplate(id: $id, input: $input) {
        id
        name
      }
    }
  `,

  DELETE_TEMPLATE: gql`
    mutation DeleteTemplate($id: ID!) {
      deleteTemplate(id: $id) {
        success
      }
    }
  `
};

// Template helper functions
export async function createTemplate(input) {
  try {
    const result = await client.request(TEMPLATE_MUTATIONS.CREATE_TEMPLATE, { input });
    return result.createTemplate;
  } catch (error) {
    console.error('Error creating template:', error);
    throw error;
  }
}

export async function getTemplates() {
  try {
    const result = await client.request(TEMPLATE_QUERIES.GET_TEMPLATES);
    return result.templates;
  } catch (error) {
    console.error('Error fetching templates:', error);
    throw error;
  }
}

export async function getTemplate(id) {
  try {
    const result = await client.request(TEMPLATE_QUERIES.GET_TEMPLATE, { id });
    return result.template;
  } catch (error) {
    console.error('Error fetching template:', error);
    throw error;
  }
}

export async function updateTemplate(id, input) {
  try {
    const result = await client.request(TEMPLATE_MUTATIONS.UPDATE_TEMPLATE, { id, input });
    return result.updateTemplate;
  } catch (error) {
    console.error('Error updating template:', error);
    throw error;
  }
}

export async function deleteTemplate(id) {
  try {
    const result = await client.request(TEMPLATE_MUTATIONS.DELETE_TEMPLATE, { id });
    return result.deleteTemplate;
  } catch (error) {
    console.error('Error deleting template:', error);
    throw error;
  }
}