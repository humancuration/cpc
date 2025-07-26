use yew::prelude::*;
use plotters::prelude::*;
use plotters_yew::Plotter;
use chrono::{DateTime, Utc};
use web_sys::{window, console};
use crate::context::Config;
use crate::ui::skeleton::SkeletonLoader;
use crate::ui::alerts::{AlertBanner, AlertBannerVariant};
use crate::ui::status::{StatusBadge, StatusBadgeVariant};
#[derive(Properties, PartialEq, Clone)]
pub struct ImpactDistributionChartProps {
    pub distribution: Vec<ImpactDistribution>,
    pub last_updated: Option<DateTime<Utc>>,
    pub is_loading: bool,
    pub loading_start: Option<f64>,
    pub error: Option<String>,
    pub on_refresh: Callback<()>,
    pub degradation_threshold: Option<f64>,
}

#[derive(Clone)]
pub struct ImpactDistribution {
    pub category: String,
    pub weight: f64,
}

fn draw_chart(
    backend: &mut dyn DrawingBackend, 
    props: &ImpactDistributionChartProps
) -> Result<(), DrawingAreaErrorKind<std::io::Error>> {
    if props.distribution.is_empty() {
        return Ok(());
    }

    let root = backend.into_drawing_area();
    root.fill(&WHITE)?;

    // Calculate total weight to determine percentages
    let total: f64 = props.distribution.iter().map(|d| d.weight).sum();

    // Validate data integrity - weights must sum to 1.0
    if (total - 1.0).abs() > f64::EPSILON {
        // Handle invalid data case with proper error visualization
        root.draw(&Text::new(
            format!("Invalid distribution data (total: {:.2})", total),
            (50, 50),
            ("sans-serif", 20).into_font().color(&RED)
        ))?;
        return Ok(());
    }

    // Prepare data for pie chart - using actual weights (not percentages)
    let pie_data: Vec<(&str, f64)> = props.distribution.iter()
        .map(|d| (d.category.as_str(), d.weight))
        .collect();

    // Create a 2D chart area for the pie chart
    let mut chart = ChartBuilder::on(&root)
        .caption("Impact Distribution", ("sans-serif", 30).into_font())
        .margin(10)
        .build_cartesian_2d(0f32..100f32, 0f32..100f32)?;

    // Hide axes since they're not needed for a pie chart
    chart.configure_mesh()
        .disable_x_axis()
        .disable_y_axis()
        .draw()?;

    // Draw the pie chart with labels
    chart.draw_series(
        PieChart::new(
            &pie_data,
            40, // radius
            &BLUE.filled(),
        )
        .border_style(&BLACK)
        .labels_style(("sans-serif", 15).into_font().color(&BLACK))
        .label_offset(20)
    )?;

    Ok(())
}

#[function_component(ImpactDistributionChart)]
pub fn impact_distribution_chart(props: &ImpactDistributionChartProps) -> Html {
    let config = use_context::<Config>().expect("Config context for impact UI thresholds");
    let thresholds = &config.impact_ui_thresholds;
    
    
    // Use degradation threshold from props if available, else from config
    let degradation_threshold = props.degradation_threshold.unwrap_or(thresholds.degradation_threshold);
    
    // Calculate elapsed time since loading started
    #[cfg(target_arch = "wasm32")]
    let elapsed = props.loading_start.map(|start| {
        window().unwrap().performance().now() - start
    }).unwrap_or(0.0);
    
    #[cfg(not(target_arch = "wasm32"))]
    let elapsed = 0.0;
    
    // Determine UI state based on elapsed time
    #[derive(PartialEq)]
    enum UIState {
        Current,
        LowLatency,
        MediumLatency,
        HighLatency,
        Error,
    }
    
    // Highlight categories above threshold
    {
        let distribution = props.distribution.clone();
        use_effect_with_deps(
            move |dist| {
                if !dist.is_empty() {
                    for category in dist {
                        let is_highlighted = category.weight > degradation_threshold;
                        // This would be connected to CSS classes in a real implementation
                        console::log_1(&format!(
                            "Category: {}, Highlighted: {}",
                            category.category,
                            is_highlighted
                        ).into());
                    }
                }
                || ()
            },
            props.distribution.clone(),
        );
    }
    
    let ui_state = if props.error.is_some() {
        UIState::Error
    } else if !props.is_loading {
        UIState::Current
    } else if elapsed < thresholds.low_latency as f64 {
        UIState::LowLatency
    } else if elapsed < thresholds.high_latency as f64 {
        UIState::MediumLatency
    } else {
        UIState::HighLatency
    };
    
    // Countdown state for high latency mode
    let countdown_seconds = use_state(|| thresholds.refresh_interval / 1000);
    
    // Reset countdown when new data arrives or error occurs
    {
        let countdown_seconds = countdown_seconds.clone();
        let refresh_interval = thresholds.refresh_interval;
        use_effect_with_deps(
            move |(last_updated, error)| {
                if last_updated.is_some() || error.is_some() {
                    countdown_seconds.set(refresh_interval / 1000);
                }
                || ()
            },
            (props.last_updated.clone(), props.error.clone()),
        );
    }
    
    // Set up countdown timer for high latency state
    {
        let countdown_seconds = countdown_seconds.clone();
        let on_refresh = props.on_refresh.clone();
        let refresh_interval = thresholds.refresh_interval;
        
        use_effect_with_deps(
            move |_| {
                if ui_state == UIState::HighLatency {
                    let window = web_sys::window().expect("should have window");
                    let interval_handle = {
                        let countdown_seconds = countdown_seconds.clone();
                        let on_refresh = on_refresh.clone();
                        window.set_interval_with_callback_and_timeout_and_arguments_0(
                            &Closure::wrap(Box::new(move || {
                                countdown_seconds.set(*countdown_seconds - 1);
                                if *countdown_seconds <= 0 {
                                    on_refresh.emit(());
                                }
                            }) as Box<dyn FnMut()>),
                            1000,
                        ).expect("set_interval should work")
                    };
                    
                    move || {
                        window.clear_interval_with_handle(interval_handle);
                    }
                } else {
                    || ()
                }
            },
            ui_state,
        );
    }
    
    // Calculate progress for medium latency state
    let progress = match ui_state {
        UIState::MediumLatency => {
            let progress = (elapsed - thresholds.low_latency as f64) /
                          (thresholds.high_latency as f64 - thresholds.low_latency as f64);
            progress.clamp(0.0, 1.0)
        }
        _ => 0.0,
    };

    let props_for_draw = props.clone();
    let drawing_function = move |backend: &mut dyn DrawingBackend| {
        draw_chart(backend, &props_for_draw)
    };

    html! {
        <div class="impact-distribution-container">
            if ui_state == UIState::HighLatency {
                <AlertBanner variant={AlertBannerVariant::Warning} class="data-delayed-banner">
                    { format!("Live data delayed. Refreshing in {}s", *countdown_seconds) }
                </AlertBanner>
            }
            
            if let Some(err) = &props.error {
                <AlertBanner variant={AlertBannerVariant::Error} class="error-banner">
                    { format!("Error: {}", err) }
                    <button onclick={props.on_refresh.clone()} class="retry-button">
                        { "Retry" }
                    </button>
                </AlertBanner>
            }
            
            <div class="chart-content">
                <div class="chart-header">
                    <h2>{ "Impact Distribution" }</h2>
                    <StatusBadge variant={StatusBadgeVariant::Live} class="timestamp" />
                </div>
                
                <div class="chart-wrapper">
                    if ui_state == UIState::MediumLatency {
                        <SkeletonLoader progress={progress} label="Data refreshing" />
                    } else if props.error.is_none() {
                        <div class={classes!("chart-visual", degradation_threshold.map(|t| format!("threshold-{}", (t * 100.0) as i32)))}>
                           <Plotter
                            width={600}
                            height={400}
                            draw_fn={Box::new(drawing_function)}
                        />
                       </div>
                    }
                </div>
                
                <div class="chart-footer">
                    if let Some(updated) = &props.last_updated {
                        { format!("Updating... {}", updated.format("%H:%M")) }
                    } else if props.is_loading {
                        { "Loading..." }
                    } else if props.error.is_some() {
                        { "Data unavailable" }
                    } else {
                        { "No data available" }
                    }
                </div>
            </div>
        </div>
    }
}