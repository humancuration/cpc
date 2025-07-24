<script>
  import { onMount } from 'svelte';
  import { toolRegistryService } from '$lib/business-tools/tool-registry.js';
  import WidgetGrid from '$lib/bi/WidgetGrid.svelte';
  import KpiWidget from '$lib/bi/KpiWidget.svelte';
  import ChartWidget from '$lib/bi/ChartWidget.svelte';
  import DataTableWidget from '$lib/bi/DataTableWidget.svelte';
  
  export let userId = null;
  
  let accountingData = {
    loading: true,
    error: null,
    summary: null,
    charts: {},
    transactions: []
  };
  
  onMount(() => {
    toolRegistryService.trackUsage('accounting-dashboard');
    loadAccountingData();
  });
  
  async function loadAccountingData() {
    try {
      // TODO: Replace with actual GraphQL query
      accountingData = {
        loading: false,
        error: null,
        summary: {
          totalRevenue: 125000,
          totalExpenses: 87500,
          netIncome: 37500,
          cashBalance: 42500,
          outstandingInvoices: 15200,
          overdueInvoices: 3200
        },
        charts: {
          revenueByMonth: [
            { month: 'Jan', revenue: 15000, expenses: 12000 },
            { month: 'Feb', revenue: 18000, expenses: 13500 },
            { month: 'Mar', revenue: 22000, expenses: 16000 },
            { month: 'Apr', revenue: 19000, expenses: 14500 },
            { month: 'May', revenue: 25000, expenses: 17500 },
            { month: 'Jun', revenue: 26000, expenses: 14000 }
          ],
          expenseCategories: [
            { category: 'Salaries', amount: 45000 },
            { category: 'Rent', amount: 12000 },
            { category: 'Supplies', amount: 8000 },
            { category: 'Marketing', amount: 5500 },
            { category: 'Utilities', amount: 3500 },
            { category: 'Other', amount: 13500 }
          ]
        },
        transactions: [
          {
            id: 1,
            date: '2024-06-15',
            description: 'Client Payment - Project Alpha',
            category: 'Revenue',
            amount: 5000,
            type: 'income'
          },
          {
            id: 2,
            date: '2024-06-14',
            description: 'Office Supplies - Staples',
            category: 'Supplies',
            amount: -245.50,
            type: 'expense'
          },
          {
            id: 3,
            date: '2024-06-13',
            description: 'Monthly Rent',
            category: 'Rent',
            amount: -2000,
            type: 'expense'
          },
          {
            id: 4,
            date: '2024-06-12',
            description: 'Software Subscription',
            category: 'Technology',
            amount: -99.99,
            type: 'expense'
          },
          {
            id: 5,
            date: '2024-06-11',
            description: 'Client Payment - Project Beta',
            category: 'Revenue',
            amount: 3500,
            type: 'income'
          }
        ]
      };
    } catch (error) {
      accountingData = {
        loading: false,
        error: error.message,
        summary: null,
        charts: {},
        transactions: []
      };
    }
  }
  
  function formatCurrency(amount) {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD'
    }).format(amount);
  }
  
  function formatPercent(value) {
    return new Intl.NumberFormat('en-US', {
      style: 'percent',
      minimumFractionDigits: 1,
      maximumFractionDigits: 1
    }).format(value / 100);
  }
</script>

<div class="accounting-dashboard">
  <div class="dashboard-header">
    <h1>Accounting Dashboard</h1>
    <p class="subtitle">Financial overview and key metrics</p>
  </div>
  
  {#if accountingData.loading}
    <div class="loading-container">
      <div class="spinner"></div>
      <p>Loading accounting data...</p>
    </div>
  {:else if accountingData.error}
    <div class="error-container">
      <h3>Error Loading Data</h3>
      <p>{accountingData.error}</p>
      <button on:click={loadAccountingData}>Retry</button>
    </div>
  {:else if accountingData.summary}
    <WidgetGrid>
      <!-- Key Financial Metrics -->
      <KpiWidget 
        title="Total Revenue" 
        value={accountingData.summary.totalRevenue} 
        format="currency"
        trend="up"
        change="+12.5%"
      />
      
      <KpiWidget 
        title="Total Expenses" 
        value={accountingData.summary.totalExpenses} 
        format="currency"
        trend="down"
        change="-5.2%"
      />
      
      <KpiWidget 
        title="Net Income" 
        value={accountingData.summary.netIncome} 
        format="currency"
        trend="up"
        change="+18.3%"
      />
      
      <KpiWidget 
        title="Cash Balance" 
        value={accountingData.summary.cashBalance} 
        format="currency"
        trend="up"
        change="+8.7%"
      />
      
      <KpiWidget 
        title="Outstanding Invoices" 
        value={accountingData.summary.outstandingInvoices} 
        format="currency"
        trend="neutral"
      />
      
      <KpiWidget 
        title="Overdue Invoices" 
        value={accountingData.summary.overdueInvoices} 
        format="currency"
        trend="down"
        change="-15.4%"
      />
      
      <!-- Charts -->
      <ChartWidget 
        type="line" 
        title="Revenue vs Expenses" 
        data={accountingData.charts.revenueByMonth}
        config={{
          xKey: 'month',
          yKeys: ['revenue', 'expenses'],
          colors: ['#10b981', '#ef4444']
        }}
      />
      
      <ChartWidget 
        type="pie" 
        title="Expense Distribution" 
        data={accountingData.charts.expenseCategories}
        config={{
          labelKey: 'category',
          valueKey: 'amount'
        }}
      />
      
      <!-- Recent Transactions -->
      <DataTableWidget 
        title="Recent Transactions" 
        data={accountingData.transactions}
        columns={[
          { key: 'date', label: 'Date', format: 'date' },
          { key: 'description', label: 'Description' },
          { key: 'category', label: 'Category' },
          { key: 'amount', label: 'Amount', format: 'currency' }
        ]}
        actions={[
          { label: 'View All', action: () => console.log('View all transactions') }
        ]}
      />
    </WidgetGrid>
  {:else}
    <div class="empty-state">
      <h3>No Accounting Data Available</h3>
      <p>Start by connecting your bank accounts or adding transactions manually.</p>
      <button on:click={loadAccountingData}>Get Started</button>
    </div>
  {/if}
</div>

<style>
  .accounting-dashboard {
    padding: 2rem;
    max-width: 1400px;
    margin: 0 auto;
  }

  .dashboard-header {
    margin-bottom: 2rem;
  }

  .dashboard-header h1 {
    font-size: 2rem;
    font-weight: 700;
    margin-bottom: 0.5rem;
    color: #1a1a1a;
  }

  .subtitle {
    color: #666;
    font-size: 1rem;
  }

  .loading-container,
  .error-container,
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 4rem 2rem;
    text-align: center;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 4px solid #f3f3f3;
    border-top: 4px solid #3498db;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin-bottom: 1rem;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  .error-container h3,
  .empty-state h3 {
    color: #e74c3c;
    margin-bottom: 0.5rem;
  }

  .empty-state h3 {
    color: #666;
  }

  button {
    background: #3498db;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.875rem;
    margin-top: 1rem;
  }

  button:hover {
    background: #2980b9;
  }

  @media (max-width: 768px) {
    .accounting-dashboard {
      padding: 1rem;
    }
  }
</style>