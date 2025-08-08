use yew::prelude::*;
use stylist::{style, yew::styled_component};
use crate::components::{metrics_card::MetricsCard, chart::{Chart, ChartDataPoint, ChartType}};

#[styled_component(DashboardPage)]
pub fn dashboard_page() -> Html {
    let page_style = style!(
        r#"
        .dashboard-page {
            max-width: 1200px;
            margin: 0 auto;
        }
        
        .page-header {
            margin-bottom: 30px;
        }
        
        .page-title {
            font-size: 2rem;
            color: #2c3e50;
            margin: 0 0 10px 0;
        }
        
        .page-description {
            color: #7f8c8d;
            margin: 0;
        }
        
        .metrics-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
        }
        
        .charts-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(500px, 1fr));
            gap: 20px;
        }
        
        @media (max-width: 768px) {
            .charts-grid {
                grid-template-columns: 1fr;
            }
        }
    "#
    ).unwrap();

    // Sample data - in a real implementation, this would come from the backend
    let engagement_data = vec![
        ChartDataPoint { label: "Jan".to_string(), value: 120.0, color: None },
        ChartDataPoint { label: "Feb".to_string(), value: 150.0, color: None },
        ChartDataPoint { label: "Mar".to_string(), value: 180.0, color: None },
        ChartDataPoint { label: "Apr".to_string(), value: 200.0, color: None },
        ChartDataPoint { label: "May".to_string(), value: 240.0, color: None },
    ];

    let retention_data = vec![
        ChartDataPoint { label: "With Viz".to_string(), value: 75.0, color: Some("#3498db".to_string()) },
        ChartDataPoint { label: "Without Viz".to_string(), value: 60.0, color: Some("#e74c3c".to_string()) },
    ];

    html! {
        <div class={page_style}>
            <div class="page-header">
                <h1 class="page-title">{"Volunteer Coordination Dashboard"}</h1>
                <p class="page-description">{"Key metrics and insights for volunteer impact"}</p>
            </div>
            
            <div class="metrics-grid">
                <MetricsCard 
                    title="Total Visualization Views".to_string()
                    value="1,842".to_string()
                    description="Total views across all volunteer visualization components".to_string()
                    trend=Some("+15% from last month".to_string())
                    color=Some("blue".to_string())
                />
                
                <MetricsCard 
                    title="Volunteer Retention Rate".to_string()
                    value="78%".to_string()
                    description="With visualization usage vs 62% without".to_string()
                    trend=Some("+16% improvement".to_string())
                    color=Some("green".to_string())
                />
                
                <MetricsCard 
                    title="Task Completion Rate".to_string()
                    value="82%".to_string()
                    description="With visualization influence vs 68% without".to_string()
                    trend=Some("+14% improvement".to_string())
                    color=Some("purple".to_string())
                />
                
                <MetricsCard 
                    title="Feedback Helpfulness".to_string()
                    value="85%".to_string()
                    description="Volunteers finding visualizations helpful".to_string()
                    trend=Some("+7% from last month".to_string())
                    color=Some("orange".to_string())
                />
            </div>
            
            <div class="charts-grid">
                <Chart 
                    title="Visualization Engagement Over Time".to_string()
                    data={engagement_data}
                    chart_type={ChartType::Line}
                    height={None}
                />
                
                <Chart 
                    title="Volunteer Retention Comparison".to_string()
                    data={retention_data}
                    chart_type={ChartType::Bar}
                    height={None}
                />
            </div>
        </div>
    }
}