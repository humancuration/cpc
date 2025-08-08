use yew::prelude::*;
use stylist::{style, yew::styled_component};
use crate::components::{metrics_card::MetricsCard, chart::{Chart, ChartDataPoint, ChartType}};

#[styled_component(AnalyticsPage)]
pub fn analytics_page() -> Html {
    let page_style = style!(
        r#"
        .analytics-page {
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
        
        .section {
            margin-bottom: 40px;
        }
        
        .section-title {
            font-size: 1.5rem;
            color: #34495e;
            margin: 0 0 20px 0;
            padding-bottom: 10px;
            border-bottom: 2px solid #ecf0f1;
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
        
        .recommendations {
            background: white;
            border-radius: 8px;
            box-shadow: 0 2px 8px rgba(0,0,0,0.1);
            padding: 20px;
        }
        
        .recommendations h3 {
            margin-top: 0;
            color: #2c3e50;
        }
        
        .recommendation-item {
            padding: 15px;
            border-left: 4px solid #3498db;
            margin-bottom: 15px;
            background-color: #f8f9fa;
        }
        
        .recommendation-item.high {
            border-left-color: #e74c3c;
        }
        
        .recommendation-item.medium {
            border-left-color: #f39c12;
        }
        
        .recommendation-item.low {
            border-left-color: #2ecc71;
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
        ChartDataPoint { label: "Skill Viz".to_string(), value: 320.0, color: None },
        ChartDataPoint { label: "Pathway Viz".to_string(), value: 280.0, color: None },
        ChartDataPoint { label: "Community Viz".to_string(), value: 190.0, color: None },
        ChartDataPoint { label: "Impact Viz".to_string(), value: 150.0, color: None },
    ];

    let feedback_data = vec![
        ChartDataPoint { label: "Helpful".to_string(), value: 78.0, color: Some("#2ecc71".to_string()) },
        ChartDataPoint { label: "Neutral".to_string(), value: 15.0, color: Some("#f39c12".to_string()) },
        ChartDataPoint { label: "Not Helpful".to_string(), value: 7.0, color: Some("#e74c3c".to_string()) },
    ];

    html! {
        <div class={page_style}>
            <div class="page-header">
                <h1 class="page-title">{"Impact Analytics"}</h1>
                <p class="page-description">{"Detailed analysis of learning impact visualization effectiveness"}</p>
            </div>
            
            <div class="section">
                <h2 class="section-title">{"Engagement Metrics"}</h2>
                <div class="metrics-grid">
                    <MetricsCard 
                        title="Avg. Interaction Time".to_string()
                        value="2m 15s".to_string()
                        description="Average time spent interacting with visualizations".to_string()
                        trend=Some("+8% from last month".to_string())
                        color=Some("blue".to_string())
                    />
                    
                    <MetricsCard 
                        title="Engagement Quality".to_string()
                        value="82%".to_string()
                        description="Quality score based on interaction depth".to_string()
                        trend=Some("+5% from last month".to_string())
                        color=Some("green".to_string())
                    />
                    
                    <MetricsCard 
                        title="Popular Visualizations".to_string()
                        value="12".to_string()
                        description="Components with above-average engagement".to_string()
                        trend=Some("+2 from last month".to_string())
                        color=Some("purple".to_string())
                    />
                </div>
                
                <div class="charts-grid">
                    <Chart 
                        title="Visualization Type Engagement".to_string()
                        data={engagement_data}
                        chart_type={ChartType::Bar}
                        height={None}
                    />
                </div>
            </div>
            
            <div class="section">
                <h2 class="section-title">{"Learning Effectiveness"}</h2>
                <div class="metrics-grid">
                    <MetricsCard 
                        title="Completion Rate".to_string()
                        value="75%".to_string()
                        description="With visualization usage vs 60% without".to_string()
                        trend=Some("+15% improvement".to_string())
                        color=Some("green".to_string())
                    />
                    
                    <MetricsCard 
                        title="Avg. Completion Time".to_string()
                        value="12 days".to_string()
                        description="With visualization vs 15 days without".to_string()
                        trend=Some("-3 days improvement".to_string())
                        color=Some("blue".to_string())
                    />
                    
                    <MetricsCard 
                        title="Satisfaction Correlation".to_string()
                        value="0.85".to_string()
                        description="Correlation between visualization use and satisfaction".to_string()
                        trend=Some("+0.05 from last month".to_string())
                        color=Some("orange".to_string())
                    />
                </div>
            </div>
            
            <div class="section">
                <h2 class="section-title">{"Community Impact"}</h2>
                <div class="metrics-grid">
                    <MetricsCard 
                        title="Volunteer Transition Rate".to_string()
                        value="25%".to_string()
                        description="Learners transitioning to volunteer activities".to_string()
                        trend=Some("+8% from last month".to_string())
                        color=Some("purple".to_string())
                    />
                    
                    <MetricsCard 
                        title="Validation Engagement".to_string()
                        value="128".to_string()
                        description="Community validation interactions".to_string()
                        trend=Some("+22 from last month".to_string())
                        color=Some("blue".to_string())
                    />
                    
                    <MetricsCard 
                        title="Impact Connection".to_string()
                        value="0.72".to_string()
                        description="Learning to community impact connection strength".to_string()
                        trend=Some("+0.07 from last month".to_string())
                        color=Some("green".to_string())
                    />
                </div>
            </div>
            
            <div class="section">
                <h2 class="section-title">{"Feedback Analysis"}</h2>
                <div class="charts-grid">
                    <Chart 
                        title="Feedback Distribution".to_string()
                        data={feedback_data}
                        chart_type={ChartType::Pie}
                        height={None}
                    />
                </div>
            </div>
            
            <div class="section">
                <h2 class="section-title">{"Educator Recommendations"}</h2>
                <div class="recommendations">
                    <div class="recommendation-item high">
                        <h3>{"High Priority: Improve Low Engagement Visualizations"}</h3>
                        <p>{"Visualization components with engagement quality scores below 0.5 need revision. Consider user research and redesign."}</p>
                    </div>
                    
                    <div class="recommendation-item medium">
                        <h3>{"Medium Priority: Enhance Course Integration"}</h3>
                        <p>{"Courses using visualizations have a 15% completion rate improvement. Consider expanding visualization integration to more courses."}</p>
                    </div>
                    
                    <div class="recommendation-item low">
                        <h3>{"Low Priority: Add More Interactive Elements"}</h3>
                        <p>{"Feedback suggests adding more interactive elements to engage users. Consider implementing drag-and-drop or click-to-explore features."}</p>
                    </div>
                </div>
            </div>
        </div>
    }
}