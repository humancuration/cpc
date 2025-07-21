use tonic::{Request, Response, Status};
use prometheus::{Encoder, TextEncoder, Registry, Gauge, Counter, Histogram, Opts, CounterVec, GaugeVec};
use std::sync::Arc;
use tokio::sync::Mutex;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Request as HyperRequest, Response as HyperResponse, Server, StatusCode,
};
use std::net::SocketAddr;
use tracing::info;

pub struct MetricsService {
    registry: Arc<Mutex<Registry>>,
    rpc_counter: CounterVec,
    rpc_success: GaugeVec,
}

impl MetricsService {
    pub fn new() -> Self {
        let registry = Registry::new();
        
        // Create RPC metrics
        let rpc_counter = CounterVec::new(
            Opts::new("rpc_calls_total", "Total RPC calls"),
            &["method"]
        ).unwrap();
        
        let rpc_success = GaugeVec::new(
            Opts::new("rpc_success", "RPC call success status"),
            &["method"]
        ).unwrap();
        
        registry.register(Box::new(rpc_counter.clone())).unwrap();
        registry.register(Box::new(rpc_success.clone())).unwrap();
        
        Self {
            registry: Arc::new(Mutex::new(registry)),
            rpc_counter,
            rpc_success,
        }
    }

    pub fn start_metrics_server(&self, addr: SocketAddr) {
        let registry = self.registry.clone();
        
        tokio::spawn(async move {
            let service = make_service_fn(move |_| {
                let registry = registry.clone();
                async move {
                    Ok::<_, hyper::Error>(service_fn(move |req: HyperRequest<Body>| {
                        handle_metrics_request(req, registry.clone())
                    }))
                }
            });
            
            let server = Server::bind(&addr).serve(service);
            info!("Metrics server running on http://{}", addr);
            
            if let Err(e) = server.await {
                tracing::error!("Metrics server error: {}", e);
            }
        });
    }

    // New instrumentation methods for middleware
    pub fn record_pre_handle(&self, method: &str) {
        self.rpc_counter.with_label_values(&[method]).inc();
    }

    pub fn record_post_handle(&self, method: &str, result: &Result<Response<impl Sized>, Status>) {
        let success_value = match result {
            Ok(_) => 1.0,
            Err(_) => 0.0,
        };
        self.rpc_success.with_label_values(&[method]).set(success_value);
    }
}

async fn handle_metrics_request(
    req: HyperRequest<Body>,
    registry: Arc<Mutex<Registry>>,
) -> Result<HyperResponse<Body>, hyper::Error> {
    if req.method() != Method::GET {
        return Ok(HyperResponse::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .unwrap());
    }
    
    let encoder = TextEncoder::new();
    let mut buffer = vec![];
    let metric_families = registry.lock().await.gather();
    
    encoder.encode(&metric_families, &mut buffer).unwrap();
    let response = HyperResponse::builder()
        .status(StatusCode::OK)
        .header(hyper::header::CONTENT_TYPE, encoder.format_type())
        .body(Body::from(buffer))
        .unwrap();
    
    Ok(response)
}