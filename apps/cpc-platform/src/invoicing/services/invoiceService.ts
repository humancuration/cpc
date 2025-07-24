// Service layer for invoice operations
import type { 
  Invoice, 
  InvoiceListResponse, 
  CreateInvoiceRequest, 
  UpdateInvoiceRequest,
  Customer,
  ApiResult
} from '../types';

// Simple mock implementation
export class InvoiceService {
  static async createInvoice(request: CreateInvoiceRequest): Promise<ApiResult<Invoice>> {
    await new Promise(resolve => setTimeout(resolve, 1000));
    return { 
      success: true, 
      data: {
        id: crypto.randomUUID(),
        number: `INV-${Date.now()}`,
        customer_id: request.customer_id,
        customer: {
          id: request.customer_id,
          name: 'Mock Customer',
          email: 'customer@example.com',
          metadata: {},
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString()
        },
        line_items: request.line_items.map(item => ({
          id: crypto.randomUUID(),
          description: item.description,
          quantity: item.quantity,
          unit_price: item.unit_price,
          total: {
            amount: (parseFloat(item.unit_price.amount) * item.quantity).toString(),
            currency: item.unit_price.currency
          },
          tax_rate: item.tax_rate,
          metadata: {}
        })),
        status: 'Draft' as any,
        issue_date: request.invoice_date,
        due_date: request.due_date,
        subtotal: {
          amount: request.line_items.reduce((sum, item) => 
            sum + parseFloat(item.unit_price.amount) * item.quantity, 0
          ).toString(),
          currency: 'USD'
        },
        tax_amount: { amount: '0', currency: 'USD' },
        total: {
          amount: request.line_items.reduce((sum, item) => 
            sum + parseFloat(item.unit_price.amount) * item.quantity, 0
          ).toString(),
          currency: 'USD'
        },
        notes: request.notes,
        terms: request.terms,
        metadata: {},
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
        sync_version: 0
      }
    };
  }

  static async getInvoice(id: string): Promise<ApiResult<Invoice | null>> {
    await new Promise(resolve => setTimeout(resolve, 500));
    return { success: true, data: null };
  }

  static async updateInvoice(request: UpdateInvoiceRequest): Promise<ApiResult<Invoice>> {
    await new Promise(resolve => setTimeout(resolve, 800));
    return { success: true, data: {} as Invoice };
  }

  static async deleteInvoice(id: string): Promise<ApiResult<void>> {
    await new Promise(resolve => setTimeout(resolve, 500));
    return { success: true, data: undefined };
  }

  static async listInvoices(
    status?: string,
    customerId?: string,
    limit = 20,
    offset = 0
  ): Promise<ApiResult<InvoiceListResponse>> {
    await new Promise(resolve => setTimeout(resolve, 600));
    
    const mockInvoices = Array.from({ length: Math.min(limit, 5) }, (_, i) => ({
      invoice: {
        id: crypto.randomUUID(),
        number: `INV-${2024}${(i + 1).toString().padStart(4, '0')}`,
        customer_id: customerId || 'mock-customer-id',
        customer: {
          id: customerId || 'mock-customer-id',
          name: 'Customer ' + (i + 1),
          email: 'customer' + (i + 1) + '@example.com',
          metadata: {},
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString()
        },
        line_items: [
          {
            id: crypto.randomUUID(),
            description: 'Service rendered',
            quantity: 1,
            unit_price: { amount: '100.00', currency: 'USD' },
            total: { amount: '100.00', currency: 'USD' },
            tax_rate: null,
            metadata: {}
          }
        ],
        status: 'Draft',
        issue_date: new Date(Date.now() - i * 24 * 60 * 60 * 1000).toISOString(),
        due_date: new Date(Date.now() + (30 - i) * 24 * 60 * 60 * 1000).toISOString(),
        subtotal: { amount: '100.00', currency: 'USD' },
        tax_amount: { amount: '0.00', currency: 'USD' },
        total: { amount: '100.00', currency: 'USD' },
        metadata: {},
        created_at: new Date(Date.now() - i * 24 * 60 * 60 * 1000).toISOString(),
        updated_at: new Date().toISOString(),
        sync_version: 0
      },
      customer: {
        id: customerId || 'mock-customer-id',
        name: 'Customer ' + (i + 1),
        email: 'customer' + (i + 1) + '@example.com',
        metadata: {},
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString()
      }
    }));
    
    return {
      success: true,
      data: {
        invoices: mockInvoices,
        total_count: 25
      }
    };
  }

  static async searchInvoices(query: string): Promise<ApiResult<InvoiceListResponse>> {
    await new Promise(resolve => setTimeout(resolve, 700));
    return {
      success: true,
      data: {
        invoices: [],
        total_count: 0
      }
    };
  }
}