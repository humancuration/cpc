# CRM Integration Plan

This document outlines integration points between the CRM module and other core modules in our cooperative platform.

## 1. CRM + Calendar Integration

### Purpose
Enable seamless scheduling of meetings with contacts directly from the CRM interface.

### Integration Points

#### Data Models
- Extend `crm_interactions` table with calendar_event_id:
```sql
ALTER TABLE crm_interactions
ADD COLUMN calendar_event_id UUID REFERENCES calendar_events(id) ON DELETE SET NULL;
```

- Add interaction_type 'calendar_event' to existing constraint:
```sql
ALTER TABLE crm_interactions
DROP CONSTRAINT crm_interactions_interaction_type_check,
ADD CONSTRAINT crm_interactions_interaction_type_check 
CHECK (interaction_type IN ('call', 'email', 'meeting', 'message', 'platform_event', 'calendar_event'));
```

#### Service Layer Integration
```rust
// In crm/application/interaction_service.rs
pub fn schedule_meeting(
    &self,
    contact_id: Uuid,
    title: String,
    description: Option<String>,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
) -> Result<Uuid> {
    // 1. Create calendar event
    let event_id = self.calendar_service.create_event(
        title,
        description,
        start_time,
        end_time,
        vec![contact_id.into()],
    )?;
    
    // 2. Create CRM interaction linking to calendar event
    let interaction = Interaction {
        id: Uuid::new_v4(),
        contact_id,
        interaction_type: InteractionType::CalendarEvent,
        calendar_event_id: Some(event_id),
        summary: title,
        details: description,
        timestamp: start_time,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    // 3. Save interaction
    self.repository.create_interaction(&interaction)?;
    
    Ok(interaction.id)
}
```

#### UI Integration
- Add "Schedule Meeting" button in contact detail view
- Show upcoming meetings in contact timeline
- Display calendar view of interactions
- Implement drag-and-drop scheduling from CRM to calendar

### Benefits
- Reduced context switching for users
- Complete interaction history in one place
- Better time management for sales teams
- Automated follow-up reminders

## 2. CRM + Health Integration

### Purpose
Track and analyze the relationship between sales team wellness and performance metrics.

### Integration Points

#### Data Models
- Create new table for sales health metrics:
```sql
CREATE TABLE sales_health_metrics (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    date DATE NOT NULL,
    stress_level SMALLINT NOT NULL CHECK (stress_level BETWEEN 0 AND 100),
    interaction_count INTEGER NOT NULL,
    deal_velocity FLOAT NOT NULL,
    conversion_rate FLOAT NOT NULL,
    PRIMARY KEY (user_id, date)
);

CREATE INDEX idx_sales_health_metrics_user ON sales_health_metrics(user_id);
CREATE INDEX idx_sales_health_metrics_date ON sales_health_metrics(date);
```

- Add wellness_score to crm_deals table:
```sql
ALTER TABLE crm_deals
ADD COLUMN wellness_score_at_deal SMALLINT 
    CHECK (wellness_score_at_deal BETWEEN 0 AND 100);
```

#### Service Layer Integration
```rust
// In crm/application/deal_service.rs
pub fn close_deal(&self, deal_id: Uuid, user_id: Uuid) -> Result<Deal> {
    // 1. Get current wellness data
    let wellness = self.health_service.get_wellness_score(user_id, Utc::now().date_naive())?;
    
    // 2. Update deal with wellness data
    let mut deal = self.repository.get_deal(deal_id)?;
    deal.wellness_score_at_deal = Some(wellness.score);
    self.repository.update_deal(&deal)?;
    
    // 3. Record sales health metric
    self.health_service.record_sales_metric(
        SalesHealthMetric {
            user_id,
            date: Utc::now().date_naive(),
            stress_level: wellness.stress_level,
            interaction_count: 1,  // This is just the current deal
            deal_velocity: self.calculate_velocity(&deal)?,
            conversion_rate: self.calculate_conversion_rate(user_id)?,
        }
    )?;
    
    // 4. Complete deal closure
    self.complete_deal_closure(deal_id)
}
```

#### UI Integration
- Add wellness insights tab in sales dashboard
- Show correlation charts between stress levels and conversion rates
- Provide proactive wellness recommendations when interaction patterns indicate stress
- Display team wellness heatmap alongside performance metrics

### Benefits
- Early detection of burnout risk in sales teams
- Data-driven wellness interventions
- Improved long-term sales performance through better wellness
- Personalized coaching based on wellness-performance correlations

## 3. CRM + Finance Integration

### Purpose
Enable revenue forecasting and financial analysis based on CRM pipeline data.

### Integration Points

#### Data Models
- Create revenue forecast models:
```sql
CREATE TABLE revenue_forecasts (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    forecast_date DATE NOT NULL,
    period_type VARCHAR(20) NOT NULL 
        CHECK (period_type IN ('daily', 'weekly', 'monthly', 'quarterly')),
    confidence_interval JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE forecast_deals (
    forecast_id UUID NOT NULL REFERENCES revenue_forecasts(id) ON DELETE CASCADE,
    deal_id UUID NOT NULL REFERENCES crm_deals(id) ON DELETE CASCADE,
    expected_value_cents BIGINT NOT NULL,
    probability INTEGER NOT NULL CHECK (probability BETWEEN 0 AND 100),
    PRIMARY KEY (forecast_id, deal_id)
);
```

- Add financial tracking to pipeline stages:
```sql
ALTER TABLE crm_pipeline_stages
ADD COLUMN expected_value_cents BIGINT,
ADD COLUMN expected_value_currency VARCHAR(3);
```

#### Service Layer Integration
```rust
// In crm/application/forecast_service.rs
pub fn generate_revenue_forecast(
    &self,
    user_id: Uuid,
    period_type: PeriodType,
    start_date: Date<Utc>,
    end_date: Date<Utc>,
) -> Result<RevenueForecast> {
    // 1. Get pipeline data
    let pipelines = self.pipeline_service.get_user_pipelines(user_id)?;
    
    // 2. Calculate expected revenue from each pipeline
    let mut forecast_deals = Vec::new();
    let mut total_expected = 0;
    
    for pipeline in pipelines {
        let stages = self.pipeline_service.get_pipeline_stages(pipeline.id)?;
        
        for stage in stages {
            let deals = self.deal_service.get_deals_in_stage(stage.id)?;
            
            for deal in deals {
                // Calculate expected value based on stage probability
                let expected_value = deal.value_cents * (stage.probability as i64) / 100;
                total_expected += expected_value;
                
                forecast_deals.push(ForecastDeal {
                    deal_id: deal.id,
                    expected_value_cents: expected_value,
                    probability: stage.probability,
                });
            }
        }
    }
    
    // 3. Create forecast record
    let forecast = RevenueForecast {
        id: Uuid::new_v4(),
        name: format!("Forecast {}-{}", start_date, end_date),
        user_id,
        forecast_date: Utc::now().date_naive(),
        period_type: period_type.to_string(),
        total_expected_cents: total_expected,
        currency: "USD".to_string(),  // Should be configurable
        deals: forecast_deals,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    // 4. Save to database
    self.repository.create_forecast(&forecast)?;
    
    // 5. Push to finance module for further analysis
    self.finance_service.import_revenue_forecast(&forecast)?;
    
    Ok(forecast)
}
```

#### UI Integration
- Add revenue forecast dashboard
- Show pipeline value vs. actual revenue
- Provide "what-if" scenario analysis
- Display financial health metrics alongside pipeline metrics
- Enable export to financial planning tools

### Benefits
- Accurate revenue forecasting based on real pipeline data
- Better financial planning and resource allocation
- Early warning for potential revenue shortfalls
- Data-driven decision making for sales and finance teams

## Implementation Strategy

1. **Phase 1: CRM + Calendar Integration**
   - Timeline: 2 weeks
   - Priority: High (immediate user value)
   - Dependencies: Calendar module must implement required API endpoints

2. **Phase 2: CRM + Finance Integration**
   - Timeline: 3 weeks
   - Priority: High (business value)
   - Dependencies: Finance module must implement forecast import endpoint

3. **Phase 3: CRM + Health Integration**
   - Timeline: 4 weeks
   - Priority: Medium (long-term value)
   - Dependencies: Health module must expose wellness metrics API

All integrations will follow our privacy-first principles with explicit user consent for data sharing between modules.