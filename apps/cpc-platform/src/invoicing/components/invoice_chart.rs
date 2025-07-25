use yew::prelude::*;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use crate::invoicing::components::dashboard::Invoice;

#[derive(Properties, PartialEq, Clone)]
pub struct InvoiceChartProps {
    pub invoices: Vec<Invoice>,
}

#[function_component(InvoiceChart)]
pub fn invoice_chart(props: &InvoiceChartProps) -> Html {
    let canvas_ref = use_node_ref();

    {
        let canvas_ref = canvas_ref.clone();
        let invoices = props.invoices.clone();
        use_effect_with_deps(move |_| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                draw_chart(canvas, &invoices);
            }
            || ()
        }, (props.invoices.clone(),));
    }

    html! {
        <div class="bg-white p-4 rounded-lg shadow">
            <h3 class="text-lg font-medium text-gray-800 mb-2">{ "Invoice Status Distribution" }</h3>
            <canvas ref={canvas_ref} width="400" height="400"></canvas>
        </div>
    }
}

fn draw_chart(canvas: HtmlCanvasElement, invoices: &[Invoice]) {
    let backend = CanvasBackend::with_canvas_object(canvas).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Invoice Status", ("sans-serif", 30).into_font())
        .build_cartesian_2d(0..1, 0..1) // The coordinate system is not used for pie charts
        .unwrap();

    chart.configure_series_labels().draw().unwrap();

    let status_counts = invoices.iter().fold(std::collections::HashMap::new(), |mut acc, invoice| {
        *acc.entry(invoice.status.clone()).or_insert(0) += 1;
        acc
    });
    
    let data: Vec<(String, i32)> = status_counts.into_iter().collect();

    let colors = [RED, GREEN, BLUE, YELLOW];
    let total: i32 = data.iter().map(|x| x.1).sum();

    let pie = Pie::new(
        &(0.5, 0.5),
        &0.4,
        data.iter().map(|(label, &value)| (label.as_str(), value as f64 / total as f64)),
    );

    chart
        .draw_series(
            pie.series()
                .zip(colors.iter())
                .map(|(shape, &color)| shape.filled(color.filled())),
        )
        .unwrap();
}