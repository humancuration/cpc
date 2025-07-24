// Service layer for customer operations
import type { Customer, CreateCustomerRequest, UpdateCustomerRequest, ApiResult } from '../types';

// Simple mock implementation
export class CustomerService {
  static async createCustomer(request: CreateCustomerRequest): Promise<ApiResult<Customer>> {
    await new Promise(resolve => setTimeout(resolve, 800));
    
    const address = request.address ? {
      street: request.address,
      city: request.city || '',
      state: request.state || '',
      postal_code: request.postal_code || '',
      country: request.country || 'US'
    } : undefined;

    return {
      success: true,
      data: {
        id: crypto.randomUUID(),
        name: request.name,
        email: request.email,
        phone: request.phone,
        address,
        tax_id: request.tax_id,
        metadata: {},
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString()
      }
    };
  }

  static async getCustomer(id: string): Promise<ApiResult<Customer | null>> {
    await new Promise(resolve => setTimeout(resolve, 400));
    return { success: true, data: null };
  }

  static async updateCustomer(request: UpdateCustomerRequest): Promise<ApiResult<Customer>> {
    await new Promise(resolve => setTimeout(resolve, 600));
    
    const address = request.address ? {
      street: request.address,
      city: request.city || '',
      state: request.state || '',
      postal_code: request.postal_code || '',
      country: request.country || 'US'
    } : undefined;

    return {
      success: true,
      data: {
        id: request.id,
        name: request.name || 'Default Name',
        email: request.email,
        phone: request.phone,
        address,
        tax_id: request.tax_id,
        metadata: {},
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString()
      }
    };
  }

  static async deleteCustomer(id: string): Promise<ApiResult<void>> {
    await new Promise(resolve => setTimeout(resolve, 400));
    return { success: true, data: undefined };
  }

  static async listCustomers(limit = 20, offset = 0): Promise<ApiResult<{ customers: Customer[]; total_count: number }>> {
    await new Promise(resolve => setTimeout(resolve, 500));
    
    const mockCustomers = Array.from({ length: Math.min(limit, 5) }, (_, i) => ({
      id: crypto.randomUUID(),
      name: 'Customer ' + (i + 1),
      email: 'customer' + (i + 1) + '@example.com',
      phone: '+1-555-' + (1000 + i),
      address: {
        street: '123 Main St',
        city: 'Anytown',
        state: 'CA',
        postal_code: '12345',
        country: 'US'
      },
      tax_id: 'TAX-' + (1000 + i),
      metadata: {},
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString()
    }));

    return {
      success: true,
      data: {
        customers: mockCustomers,
        total_count: mockCustomers.length
      }
    };
  }
}