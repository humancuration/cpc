# Revenue Dashboard

**Author:** Kai Tanaka  
**Date:** 2025-07-23  
**Status:** Technical Specification  
**Version:** 1.0.0

## Overview

The Revenue Dashboard provides comprehensive revenue tracking and analysis capabilities for cooperatives and businesses within the CPC ecosystem. It offers real-time revenue monitoring, multi-source income tracking, predictive analytics, and detailed financial performance insights with support for various revenue models including subscriptions, one-time sales, donations, and cooperative dividends.

## Component Specifications

### Frontend Components (Yew)

#### 1. RevenueDashboard.svelte
**Purpose**: Main container for revenue analytics interface
- **Props**:
  - `entityId: string` - Cooperative or business identifier
  - `dateRange: DateRange` - Analysis period
  - `granularity: Granularity` - Daily/weekly/monthly aggregation
- **Features**:
  - Responsive widget grid layout
  - Real-time revenue updates via subscriptions
  - Interactive date range picker
  - Currency conversion support
  - Export functionality (PDF reports, Excel data)

#### 2. RevenueKPIs.svelte
**Purpose**: Display key revenue performance indicators
- **Metrics Displayed**:
  - Total Revenue (period)
  - Monthly Recurring Revenue (MRR)
  - Average Revenue Per User (ARPU)
  - Revenue Growth Rate
  - Customer Lifetime Value (CLV)
  - Churn Rate Impact
- **Features**:
  - Trend indicators with period-over-period comparison
  - Drill-down capability to detailed views
  - Customizable KPI selection
  - Alert thresholds configuration

#### 3. RevenueStreams.svelte
**Purpose**: Visual breakdown of revenue sources
- **Visualization Types**:
  - Waterfall charts for revenue composition
  - Sankey diagrams for revenue flow
  - Stacked area charts for temporal trends
  - Donut charts for proportional analysis
- **Revenue Categories**:
  - Subscription revenue
  - One-time sales
  - Service fees
  - Donations/grants
  - Investment income
  - Cooperative dividends

#### 4. CustomerRevenue.svelte
**Purpose**: Customer-centric revenue analysis
- **Features**:
  - Customer segmentation by revenue contribution
  - Cohort analysis visualization
  - Churn prediction indicators
  - Upsell opportunity identification
  - Revenue concentration analysis (top 20% customers)

### Backend Services (Rust/Axum)

#### 1. Revenue Service
**Location**: `apps/backend/src/services/revenue_service.rs`

```rust
pub struct RevenueService {
    db: Arc<Database>,
    analytics_service: Arc<AnalyticsService>,
    prediction_service: Arc<PredictionService>,
}

impl RevenueService {
    pub async fn get_revenue_summary(
        &self, 
        entity_id: Uuid, 
        period: DateRange
    ) -> Result<RevenueSummary, Error>;
    
    pub async fn get_revenue_streams(
        &self, 
        entity_id: Uuid, 
        period: DateRange
    ) -> Result<Vec<RevenueStream>, Error>;
    
    pub async fn get_customer_revenue(
        &self, 
        entity_id: Uuid, 
        period: DateRange
    ) -> Result<Vec<CustomerRevenue>, Error>;
    
    pub async fn predict_revenue(
        &self, 
        entity_id: Uuid, 
        months: i32
    ) -> Result<RevenuePrediction, Error>;
}
```

#### 2. MRR Calculator Service
```rust
pub struct MrrService {
    db: Arc<Database>,
}

impl MrrService {
    pub async fn calculate_mrr(&self, entity_id: Uuid) -> Result<Decimal, Error>;
    pub async fn calculate_arr(&self, entity_id: Uuid) -> Result<Decimal, Error>;
    pub async fn get_mrr_trend(&self, entity_id: Uuid, months: i32) -> Result<Vec<MrrDataPoint>, Error>;
}
```

#### 3. Churn Analysis Service
```rust
pub struct ChurnAnalysisService {
    db: Arc<Database>,
    ml_service: Arc<MLService>,
}

impl ChurnAnalysisService {
    pub async fn calculate_churn_rate(&self, entity_id: Uuid, period: DateRange) -> Result<ChurnMetrics, Error>;
    pub async fn predict_churn_risk(&self, customer_id: Uuid) -> Result<ChurnRisk, Error>;
}
```

## Data Models

### Core Entities

#### RevenueTransaction
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueTransaction {
    pub id: Uuid,
    pub entity_id: Uuid, // cooperative or business
    pub customer_id: Option<Uuid>,
    pub amount: Decimal,
    pub currency: String,
    pub revenue_type: RevenueType,
    pub subscription_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub service_id: Option<Uuid>,
    pub transaction_date: DateTime<Utc>,
    pub recognition_date: DateTime<Utc>,
    pub status: TransactionStatus,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}
```

#### SubscriptionRevenue
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionRevenue {
    pub id: Uuid,
    pub subscription_id: Uuid,
    pub customer_id: Uuid,
    pub plan_id: Uuid,
    pub amount: Decimal,
    pub currency: String,
    pub billing_period: BillingPeriod,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub status: SubscriptionStatus,
}
```

#### RevenueRecognitionRule
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueRecognitionRule {
    pub id: Uuid,
    pub entity_id: Uuid,
    pub rule_type: RecognitionType,
    pub conditions: serde_json::Value,
    def revenue_schedule: Vec<RecognitionSchedule>,
}
```

### Database Schema

```sql
-- Revenue transactions
CREATE TABLE revenue_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_id UUID NOT NULL REFERENCES cooperatives(id),
    customer_id UUID REFERENCES customers(id),
    amount DECIMAL(12,2) NOT NULL,
    currency VARCHAR(3) NOT NULL,
    revenue_type VARCHAR(50) NOT NULL,
    subscription_id UUID REFERENCES subscriptions(id),
    product_id UUID REFERENCES products(id),
    service_id UUID REFERENCES services(id),
    transaction_date TIMESTAMPTZ NOT NULL,
    recognition_date TIMESTAMPTZ NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'completed',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Subscription revenue tracking
CREATE TABLE subscription_revenue (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    subscription_id UUID NOT NULL REFERENCES subscriptions(id),
    customer_id UUID NOT NULL REFERENCES customers(id),
    plan_id UUID NOT NULL REFERENCES subscription_plans(id),
    amount DECIMAL(12,2) NOT NULL,
    currency VARCHAR(3) NOT NULL,
    billing_period VARCHAR(20) NOT NULL,
    start_date TIMESTAMPTZ NOT NULL,
    end_date TIMESTAMPTZ,
    status VARCHAR(20) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Revenue recognition rules
CREATE TABLE revenue_recognition_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_id UUID NOT NULL REFERENCES cooperatives(id),
    rule_type VARCHAR(50) NOT NULL,
    conditions JSONB NOT NULL,
    revenue_schedule JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Revenue categories
CREATE TABLE revenue_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_id UUID NOT NULL REFERENCES cooperatives(id),
    name VARCHAR(100) NOT NULL,
    category_type VARCHAR(50) NOT NULL,
    parent_id UUID REFERENCES revenue_categories(id),
    is_system BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_revenue_entity_date ON revenue_transactions(entity_id, recognition_date DESC);
CREATE INDEX idx_revenue_customer ON revenue_transactions(customer_id);
CREATE INDEX idx_revenue_type ON revenue_transactions(revenue_type);
CREATE INDEX idx_subscription_revenue_customer ON subscription_revenue(customer_id);
```

## API Contracts

### GraphQL Schema Extensions

```graphql
type RevenueTransaction {
  id: ID!
  entityId: ID!
  customer: Customer
  amount: Float!
  currency: String!
  revenueType: RevenueType!
  subscription: Subscription
  product: Product
  service: Service
  transactionDate: DateTime!
  recognitionDate: DateTime!
  status: TransactionStatus!
  metadata: JSON
}

type RevenueSummary {
  totalRevenue: Float!
  mrr: Float
  arr: Float
  growthRate: Float
  churnRate: Float
  arpu: Float
  clv: Float
  period: DateRange!
}

type RevenueStream {
  id: ID!
  name: String!
  type: RevenueType!
  amount: Float!
  percentage: Float!
  trend: TrendData!
  customers: Int!
}

type CustomerRevenue {
  customerId: ID!
  customerName: String!
  totalRevenue: Float!
  mrr: Float
  lastTransactionDate: DateTime
  churnRisk: Float
  segment: CustomerSegment!
}

type RevenuePrediction {
  predictedRevenue: Float!
  confidence: Float!
  factors: [PredictionFactor!]!
  monthlyBreakdown: [MonthlyPrediction!]!
}

enum RevenueType {
  SUBSCRIPTION
  ONE_TIME_SALE
  SERVICE_FEE
  DONATION
  INVESTMENT_INCOME
  DIVIDEND
  OTHER
}

type Query {
  revenueTransactions(filter: RevenueFilter, pagination: PaginationInput): RevenueConnection!
  revenueSummary(entityId: ID!, period: DateRange!): RevenueSummary!
  revenueStreams(entityId: ID!, period: DateRange!): [RevenueStream!]!
  customerRevenue(entityId: ID!, period: DateRange!): [CustomerRevenue!]!
  revenuePrediction(entityId: ID!, months: Int!): RevenuePrediction!
  mrrBreakdown(entityId: ID!): MrrBreakdown!
}

type Mutation {
  recordRevenue(input: RecordRevenueInput!): RevenueTransaction!
  updateRevenue(id: ID!, input: UpdateRevenueInput!): RevenueTransaction!
  deleteRevenue(id: ID!): Boolean!
}

type Subscription {
  revenueUpdated(entityId: ID!): RevenueUpdate!
  mrrChanged(entityId: ID!): MrrUpdate!
}
```

### REST API Endpoints

#### GET /api/v1/revenue/summary
Get comprehensive revenue summary for specified period.

**Query Parameters**:
- `entity_id` (required): Cooperative or business ID
- `start_date` (required): ISO 8601 start date
- `end_date` (required): ISO 8601 end date
- `currency` (optional): Currency code (default: USD)
- `include_predictions` (optional): Include ML predictions

**Response**:
```json
{
  "summary": {
    "totalRevenue": 125000.00,
    "mrr": 15000.00,
    "arr": 180000.00,
    "growthRate": 0.15,
    "churnRate": 0.05,
    "arpu": 125.50,
    "clv": 1500.00
  },
  "breakdown": {
    "subscription": 80000.00,
    "oneTime": 30000.00,
    "service": 15000.00
  },
  "period": {
    "start": "2025-01-01",
    "end": "2025-12-31"
  }
}
```

#### POST /api/v1/revenue/record
Record new revenue transaction.

**Request Body**:
```json
{
  "entityId": "550e8400-e29b-41d4-a716-446655440000",
  "amount": 5000.00,
  "currency": "USD",
  "revenueType": "SUBSCRIPTION",
  "customerId": "660e8400-e29b-41d4-a716-446655440001",
  "subscriptionId": "770e8400-e29b-41d4-a716-446655440002",
  "recognitionDate": "2025-07-23",
  "metadata": {
    "plan": "premium",
    "billingCycle": "annual"
  }
}
```

## Integration Guidelines

### Frontend Integration

#### 1. Revenue Dashboard Component
```svelte
<script>
  import { onMount } from 'svelte';
  import { graphqlClient } from '$lib/graphql/client';
  import { REVENUE_SUMMARY, REVENUE_STREAMS } from '$lib/graphql/queries';
  import RevenueKPIs from '$lib/components/RevenueKPIs.svelte';
  import RevenueStreams from '$lib/components/RevenueStreams.svelte';
  
  export let entityId;
  
  let summary = null;
  let streams = [];
  
  onMount(async () => {
    const [summaryResult, streamsResult] = await Promise.all([
      graphqlClient.query({
        query: REVENUE_SUMMARY,
        variables: { entityId, period: { start: '2025-01-01', end: '2025-12-31' } }
      }),
      graphqlClient.query({
        query: REVENUE_STREAMS,
        variables: { entityId, period: { start: '2025-01-01', end: '2025-12-31' } }
      })
    ]);
    
    summary = summaryResult.data.revenueSummary;
    streams = streamsResult.data.revenueStreams;
  });
</script>

<div class="revenue-dashboard">
  <RevenueKPIs data={summary} />
  <RevenueStreams data={streams} />
</div>
```

#### 2. Real-time Updates
```javascript
// stores/revenueStore.js
import { writable } from 'svelte/store';
import { graphqlClient } from '$lib/graphql/client';
import { REVENUE_SUBSCRIPTION } from '$lib/graphql/subscriptions';

function createRevenueStore() {
  const { subscribe, update } = writable({
    summary: null,
    streams: [],
    loading: true
  });

  let unsubscribe = null;

  return {
    subscribe,
    async initialize(entityId) {
      // Initial data fetch
      const data = await fetchRevenueData(entityId);
      update(state => ({ ...state, ...data, loading: false }));
      
      // Subscribe to real-time updates
      unsubscribe = graphqlClient.subscribe({
        query: REVENUE_SUBSCRIPTION,
        variables: { entityId }
      }).subscribe({
        next: ({ data }) => {
          update(state => ({
            ...state,
            summary: data.revenueUpdated.summary,
            streams: data.revenueUpdated.streams
          }));
        }
      });
    },
    destroy() {
      if (unsubscribe) unsubscribe();
    }
  };
}
```

### Backend Integration

#### 1. Service Implementation
```rust
// backend/src/services/revenue_service.rs
use async_graphql::*;
use sqlx::PgPool;

pub struct RevenueService {
    pool: PgPool,
}

impl RevenueService {
    pub async fn get_revenue_summary(
        &self,
        entity_id: Uuid,
        period: DateRange
    ) -> Result<RevenueSummary> {
        let total_revenue = sqlx::query_scalar!(
            r#"
            SELECT COALESCE(SUM(amount), 0)
            FROM revenue_transactions
            WHERE entity_id = $1
              AND recognition_date >= $2
              AND recognition_date <= $3
              AND status = 'completed'
            "#,
            entity_id,
            period.start,
            period.end
        )
        .fetch_one(&self.pool)
        .await?;

        let mrr = self.calculate_mrr(entity_id).await?;
        
        Ok(RevenueSummary {
            total_revenue,
            mrr,
            arr: mrr * 12,
            growth_rate: self.calculate_growth_rate(entity_id, &period).await?,
            churn_rate: self.calculate_churn_rate(entity_id, &period).await?,
            arpu: self.calculate_arpu(entity_id, &period).await?,
            clv: self.calculate_clv(entity_id).await?,
            period,
        })
    }

    async fn calculate_mrr(&self, entity_id: Uuid) -> Result<Decimal> {
        sqlx::query_scalar!(
            r#"
            SELECT COALESCE(SUM(amount), 0)
            FROM subscription_revenue
            WHERE entity_id = $1
              AND status = 'active'
              AND end_date IS NULL
            "#,
            entity_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }
}
```

### Third-Party Integrations

#### 1. Payment Processors
- **Stripe Integration**: Automatic revenue recording from Stripe webhooks
- **PayPal**: Transaction import via PayPal API
- **Bank APIs**: Direct bank feed integration for investment income

#### 2. Accounting Systems
- **QuickBooks**: Two-way sync for revenue recognition
- **Xero**: API integration for financial reporting
- **Sage**: Custom connector for enterprise clients

#### 3. Analytics Platforms
- **Google Analytics**: E-commerce tracking integration
- **Mixpanel**: Customer behavior correlation
- **Segment**: Unified customer data platform

### Performance Optimization

#### 1. Caching Strategy
```rust
// Cache revenue summaries for 1 hour
const REVENUE_CACHE_TTL: Duration = Duration::from_secs(3600);

pub async fn get_cached_revenue_summary(
    &self,
    entity_id: Uuid,
    period: DateRange
) -> Result<RevenueSummary> {
    let cache_key = format!("revenue:{}:{}:{}", entity_id, period.start, period.end);
    
    if let Some(cached) = self.cache.get(&cache_key).await {
        return Ok(cached);
    }
    
    let summary = self.calculate_revenue_summary(entity_id, period).await?;
    self.cache.set(&cache_key, &summary, REVENUE_CACHE_TTL).await?;
    
    Ok(summary)
}
```

#### 2. Database Optimization
- **Materialized Views**: Pre-aggregated revenue data
- **Partitioning**: Monthly partitioning for large datasets
- **Read Replicas**: Separate read instances for analytics

### Security Considerations

#### 1. Data Access Control
```rust
pub async fn check_revenue_access(
    &self,
    user_id: Uuid,
    entity_id: Uuid
) -> Result<bool> {
    sqlx::query_scalar!(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM cooperative_members
            WHERE user_id = $1 AND cooperative_id = $2
            AND role IN ('admin', 'finance')
        )
        "#,
        user_id,
        entity_id
    )
    .fetch_one(&self.pool)
    .await
    .map_err(Into::into)
}
```

#### 2. Audit Trail
```rust
pub async fn log_revenue_access(
    &self,
    user_id: Uuid,
    entity_id: Uuid,
    action: &str
) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO revenue_audit_log (user_id, entity_id, action, timestamp)
        VALUES ($1, $2, $3, NOW())
        "#,
        user_id,
        entity_id,
        action
    )
    .execute(&self.pool)
    .await?;
    
    Ok(())
}
```

## Monitoring and Alerting

### Key Metrics
- **Revenue Growth Rate**: Daily tracking with 5% threshold alerts
- **MRR Churn**: Monthly churn rate > 5% triggers alert
- **Failed Transactions**: Real-time monitoring with immediate alerts
- **API Latency**: P95 response time < 500ms

### Alert Configuration
```yaml
alerts:
  - name: revenue_drop
    condition: daily_revenue < previous_day * 0.9
    severity: warning
    channels: [email, slack]
  
  - name: mrr_churn_high
    condition: monthly_churn_rate > 0.05
    severity: critical
    channels: [email, slack, pagerduty]
```

## Future Enhancements

### Advanced Analytics
1. **AI Revenue Forecasting**: ML models for 12-month predictions
2. **Customer Segmentation**: Advanced clustering for revenue optimization
3. **Pricing Optimization**: Dynamic pricing recommendations
4. **Revenue Attribution**: Multi-touch attribution modeling

### Integration Roadmap
1. **CRM Integration**: Salesforce, HubSpot connectivity
2. **ERP Systems**: SAP, Oracle integration
3. **Marketplace APIs**: Shopify, WooCommerce revenue sync
4. **Subscription Management**: Chargebee, Recurly integration