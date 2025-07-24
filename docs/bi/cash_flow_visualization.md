# Cash Flow Visualization System

**Author:** Kai Tanaka  
**Date:** 2025-07-23  
**Status:** Technical Specification  
**Version:** 1.0.0

## Overview

The Cash Flow Visualization System provides comprehensive cash flow analysis and forecasting capabilities for cooperatives and businesses within the CPC ecosystem. It enables real-time tracking of cash inflows and outflows, liquidity analysis, runway calculations, and predictive cash flow modeling with support for multiple accounts, currencies, and scenario planning.

## Component Specifications

### Frontend Components (Yew)

#### 1. CashFlowDashboard.svelte
**Purpose**: Main container for cash flow analytics interface
- **Props**:
  - `entityId: string` - Cooperative or business identifier
  - `accounts: string[]` - Optional account filtering
  - `forecastMonths: number` - Forecast period (default: 12)
  - `currency: string` - Display currency
- **Features**:
  - Interactive cash flow timeline
  - Real-time balance updates
  - Multi-account aggregation
  - Currency conversion
  - Scenario planning interface
  - Export functionality (PDF, Excel, CSV)

#### 2. CashFlowChart.svelte
**Purpose**: Visual cash flow representation
- **Visualization Types**:
  - **Waterfall Chart**: Monthly cash flow breakdown
  - **Area Chart**: Cumulative cash position
  - **Heatmap**: Daily cash flow patterns
  - **Sankey Diagram**: Cash flow sources and uses
  - **Forecast Bands**: Predictive ranges with confidence intervals
- **Interactive Features**:
  - Hover tooltips with detailed breakdown
  - Click-to-drill-down on any data point
  - Zoom and pan capabilities
  - Custom date range selection
  - Real-time updates via WebSocket

#### 3. LiquidityWidget.svelte
**Purpose**: Display liquidity metrics and alerts
- **Metrics Displayed**:
  - Current Ratio
  - Quick Ratio
  - Cash Ratio
  - Days Cash on Hand
  - Burn Rate
  - Runway (months)
  - Working Capital
- **Alert System**:
  - Low balance warnings
  - Negative cash flow alerts
  - Runway threshold notifications
  - Custom alert configuration

#### 4. CashFlowForecast.svelte
**Purpose**: Predictive cash flow modeling
- **Features**:
  - ML-based forecasting with confidence intervals
  - Scenario planning (best/worst/base case)
  - What-if analysis tools
  - Seasonal adjustment
  - External factor integration
  - Manual override capabilities

### Backend Services (Rust/Axum)

#### 1. CashFlow Service
**Location**: `apps/backend/src/services/cash_flow_service.rs`

```rust
pub struct CashFlowService {
    db: Arc<Database>,
    forecast_service: Arc<ForecastService>,
    bank_service: Arc<BankIntegrationService>,
    ml_service: Arc<MLPredictionService>,
}

impl CashFlowService {
    pub async fn get_cash_flow(
        &self,
        entity_id: Uuid,
        period: DateRange,
        accounts: Option<Vec<Uuid>>
    ) -> Result<CashFlowData, Error>;
    
    pub async fn get_liquidity_metrics(
        &self,
        entity_id: Uuid
    ) -> Result<LiquidityMetrics, Error>;
    
    pub async fn forecast_cash_flow(
        &self,
        entity_id: Uuid,
        months: i32,
        scenarios: Vec<Scenario>
    ) -> Result<CashFlowForecast, Error>;
    
    pub async fn get_runway(
        &self,
        entity_id: Uuid
    ) -> Result<RunwayCalculation, Error>;
}
```

#### 2. Transaction Classification Service
```rust
pub struct TransactionClassifier {
    ml_model: Arc<TransactionMLModel>,
    rules_engine: Arc<RulesEngine>,
}

impl TransactionClassifier {
    pub async fn classify_transaction(
        &self,
        transaction: Transaction
    ) -> Result<TransactionClassification, Error>;
    
    pub async fn get_cash_flow_categories(
        &self,
        entity_id: Uuid
    ) -> Result<Vec<CashFlowCategory>, Error>;
}
```

#### 3. Bank Integration Service
```rust
pub struct BankIntegrationService {
    providers: HashMap<String, Arc<dyn BankProvider>>,
    sync_service: Arc<AccountSyncService>,
}

impl BankIntegrationService {
    pub async fn sync_account_balances(
        &self,
        entity_id: Uuid
    ) -> Result<Vec<AccountBalance>, Error>;
    
    pub async fn get_real_time_balances(
        &self,
        entity_id: Uuid
    ) -> Result<HashMap<Uuid, Decimal>, Error>;
}
```

## Data Models

### Core Entities

#### CashFlowTransaction
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashFlowTransaction {
    pub id: Uuid,
    pub entity_id: Uuid,
    pub account_id: Uuid,
    pub amount: Decimal,
    pub currency: String,
    pub transaction_type: TransactionType,
    pub category: CashFlowCategory,
    pub subcategory: String,
    pub description: String,
    pub transaction_date: DateTime<Utc>,
    pub effective_date: DateTime<Utc>,
    pub reference: Option<String>,
    pub counterparty: Option<String>,
    pub tags: Vec<String>,
    pub is_recurring: bool,
    pub recurring_rule: Option<RecurringRule>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}
```

#### AccountBalance
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalance {
    pub account_id: Uuid,
    pub balance: Decimal,
    pub available_balance: Decimal,
    pub currency: String,
    pub last_updated: DateTime<Utc>,
    pub pending_transactions: Vec<PendingTransaction>,
}
```

#### CashFlowForecast
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashFlowForecast {
    pub entity_id: Uuid,
    pub forecast_period: DateRange,
    pub base_case: Vec<CashFlowProjection>,
    pub optimistic_case: Vec<CashFlowProjection>,
    pub pessimistic_case: Vec<CashFlowProjection>,
    pub confidence_intervals: ConfidenceIntervals,
    pub influencing_factors: Vec<ForecastFactor>,
}
```

### Database Schema

```sql
-- Cash flow transactions
CREATE TABLE cash_flow_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_id UUID NOT NULL REFERENCES cooperatives(id),
    account_id UUID NOT NULL REFERENCES bank_accounts(id),
    amount DECIMAL(12,2) NOT NULL,
    currency VARCHAR(3) NOT NULL,
    transaction_type VARCHAR(20) NOT NULL CHECK (transaction_type IN ('INFLOW', 'OUTFLOW')),
    category VARCHAR(50) NOT NULL,
    subcategory VARCHAR(100),
    description TEXT NOT NULL,
    transaction_date TIMESTAMPTZ NOT NULL,
    effective_date TIMESTAMPTZ NOT NULL,
    reference VARCHAR(255),
    counterparty VARCHAR(255),
    tags TEXT[] DEFAULT '{}',
    is_recurring BOOLEAN DEFAULT FALSE,
    recurring_rule JSONB,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Bank accounts
CREATE TABLE bank_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_id UUID NOT NULL REFERENCES cooperatives(id),
    provider VARCHAR(50) NOT NULL,
    account_number VARCHAR(100),
    account_name VARCHAR(255) NOT NULL,
    account_type VARCHAR(50) NOT NULL,
    currency VARCHAR(3) NOT NULL,
    current_balance DECIMAL(12,2),
    available_balance DECIMAL(12,2),
    last_sync TIMESTAMPTZ,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Cash flow categories
CREATE TABLE cash_flow_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_id UUID NOT NULL REFERENCES cooperatives(id),
    name VARCHAR(100) NOT NULL,
    category_type VARCHAR(20) NOT NULL CHECK (category_type IN ('OPERATING', 'INVESTING', 'FINANCING')),
    parent_id UUID REFERENCES cash_flow_categories(id),
    is_system BOOLEAN DEFAULT FALSE,
    color VARCHAR(7),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Cash flow forecasts
CREATE TABLE cash_flow_forecasts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_id UUID NOT NULL REFERENCES cooperatives(id),
    forecast_start DATE NOT NULL,
    forecast_end DATE NOT NULL,
    forecast_data JSONB NOT NULL,
    confidence_level FLOAT NOT NULL,
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_cash_flow_entity_date ON cash_flow_transactions(entity_id, effective_date DESC);
CREATE INDEX idx_cash_flow_account ON cash_flow_transactions(account_id);
CREATE INDEX idx_cash_flow_category ON cash_flow_transactions(category);
CREATE INDEX idx_cash_flow_type ON cash_flow_transactions(transaction_type);
```

## API Contracts

### GraphQL Schema Extensions

```graphql
type CashFlowTransaction {
  id: ID!
  entityId: ID!
  account: BankAccount!
  amount: Float!
  currency: String!
  transactionType: TransactionType!
  category: CashFlowCategory!
  subcategory: String
  description: String!
  transactionDate: DateTime!
  effectiveDate: DateTime!
  reference: String
  counterparty: String
  tags: [String!]!
  isRecurring: Boolean!
  metadata: JSON
}

type BankAccount {
  id: ID!
  entityId: ID!
  provider: String!
  accountName: String!
  accountType: String!
  currency: String!
  currentBalance: Float!
  availableBalance: Float!
  lastSync: DateTime
  isActive: Boolean!
}

type CashFlowSummary {
  entityId: ID!
  period: DateRange!
  openingBalance: Float!
  closingBalance: Float!
  totalInflows: Float!
  totalOutflows: Float!
  netCashFlow: Float!
  operatingCashFlow: Float!
  investingCashFlow: Float!
  financingCashFlow: Float!
}

type LiquidityMetrics {
  currentRatio: Float!
  quickRatio: Float!
  cashRatio: Float!
  daysCashOnHand: Int!
  burnRate: Float!
  runway: Float!
  workingCapital: Float!
}

type CashFlowForecast {
  entityId: ID!
  forecastPeriod: DateRange!
  projections: [CashFlowProjection!]!
  confidenceIntervals: ConfidenceIntervals!
  influencingFactors: [ForecastFactor!]!
}

type CashFlowProjection {
  date: DateTime!
  predictedInflow: Float!
  predictedOutflow: Float!
  predictedBalance: Float!
  confidence: Float!
}

enum TransactionType {
  INFLOW
  OUTFLOW
}

enum CashFlowCategoryType {
  OPERATING
  INVESTING
  FINANCING
}

type Query {
  cashFlowTransactions(filter: CashFlowFilter, pagination: PaginationInput): CashFlowConnection!
  cashFlowSummary(entityId: ID!, period: DateRange!): CashFlowSummary!
  liquidityMetrics(entityId: ID!): LiquidityMetrics!
  cashFlowForecast(entityId: ID!, months: Int!, scenarios: [ScenarioInput!]): CashFlowForecast!
  accountBalances(entityId: ID!): [AccountBalance!]!
  runwayCalculation(entityId: ID!): RunwayCalculation!
}

type Mutation {
  recordCashFlow(input: RecordCashFlowInput!): CashFlowTransaction!
  updateCashFlow(id: ID!, input: UpdateCashFlowInput!): CashFlowTransaction!
  deleteCashFlow(id: ID!): Boolean!
  syncBankAccounts(entityId: ID!): SyncResult!
}

type Subscription {
  cashFlowUpdated(entityId: ID!): CashFlowUpdate!
  balanceChanged(entityId: ID!): BalanceUpdate!
}
```

### REST API Endpoints

#### GET /api/v1/cash-flow/summary
Get comprehensive cash flow summary for specified period.

**Query Parameters**:
- `entity_id` (required): Cooperative or business ID
- `start_date` (required): ISO 8601 start date
- `end_date` (required): ISO 8601 end date
- `accounts` (optional): Comma-separated account IDs
- `currency` (optional): Currency for conversion

**Response**:
```json
{
  "summary": {
    "period": {
      "start": "2025-01-01",
      "end": "2025-12-31"
    },
    "openingBalance": 50000.00,
    "closingBalance": 75000.00,
    "totalInflows": 250000.00,
    "totalOutflows": 225000.00,
    "netCashFlow": 25000.00,
    "operatingCashFlow": 180000.00,
    "investingCashFlow": -50000.00,
    "financingCashFlow": 45000.00
  },
  "byCategory": {
    "operating": {
      "inflows": 200000.00,
      "outflows": 150000.00
    },
    "investing": {
      "inflows": 5000.00,
      "outflows": 55000.00
    },
    "financing": {
      "inflows": 45000.00,
      "outflows": 20000.00
    }
  }
}
```

#### POST /api/v1/cash-flow/forecast
Generate cash flow forecast with scenarios.

**Request Body**:
```json
{
  "entityId": "550e8400-e29b-41d4-a716-446655440000",
  "months": 12,
  "scenarios": [
    {
      "name": "Conservative Growth",
      "assumptions": {
        "revenueGrowth": 0.05,
        "expenseGrowth": 0.03
      }
    },
    {
      "name": "Aggressive Expansion",
      "assumptions": {
        "revenueGrowth": 0.25,
        "expenseGrowth": 0.15
      }
    }
  ]
}
```

## Integration Guidelines

### Frontend Integration

#### 1. Cash Flow Dashboard Component
```svelte
<script>
  import { onMount } from 'svelte';
  import { cashFlowStore } from '$lib/stores/cashFlowStore';
  import CashFlowChart from '$lib/components/CashFlowChart.svelte';
  import LiquidityWidget from '$lib/components/LiquidityWidget.svelte';
  
  export let entityId;
  
  let summary = null;
  let forecast = null;
  
  onMount(async () => {
    await cashFlowStore.initialize(entityId);
    summary = await cashFlowStore.getSummary({
      start: '2025-01-01',
      end: '2025-12-31'
    });
    forecast = await cashFlowStore.getForecast(entityId, 12);
  });
</script>

<div class="cash-flow-dashboard">
  <LiquidityWidget data={summary} />
  <CashFlowChart 
    data={summary}
    forecast={forecast}
  />
</div>
```

#### 2. Real-time Balance Updates
```javascript
