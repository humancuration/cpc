use yew::prelude::*;
use yew_hooks::use_async;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlVideoElement, HtmlCanvasElement};
use crate::graphql::product::ProductByBarcode;
use crate::store::product::ProductStore;
use crate::components::common::buttons::Button;
use crate::services::barcode::BarcodeService;
use crate::services::camera::{CameraService, CameraPermission};
use yewdux::prelude::*;
use tracing::{info, error};
use crate::types::product::{ProductResponse, BarcodeError, BarcodeErrorCode};

#[wasm_bindgen]
extern "C" {
    // External JS function to initialize camera
    #[wasm_bindgen(js_namespace = ["window", "camera"])]
    fn start_camera(video_id: &str) -> Result<(), JsValue>;
}

pub enum Msg {
    StartScan,
    CheckCamera(Result<(), BarcodeError>),
    BarcodeDetected(String),
    ScanResult(Result<ProductResponse, BarcodeError>),
}

#[derive(PartialEq)]
pub enum State {
    Idle,
    CheckingCamera,
    PermissionDenied,
    CameraNotAvailable,
    CameraActive,
    Scanning,
    Success(ProductResponse),
    Error(BarcodeError),
}

pub struct BarcodeScanner {
    video_ref: NodeRef,
    canvas_ref: NodeRef,
    scan_task: Option<gloo_timers::future::TimeoutFuture>,
    state: State,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub on_product_found: Callback<ProductResponse>,
    pub on_error: Callback<BarcodeError>,
}

impl Component for BarcodeScanner {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            video_ref: NodeRef::default(),
            canvas_ref: NodeRef::default(),
            scan_task: None,
            state: State::Idle,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StartScan => {
                info!("Starting barcode scan");
                self.state = State::CheckingCamera;
                
                // Check camera permission and availability
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match CameraService::ensure_permission().await {
                        Ok(_) => {
                            match CameraService::check_availability().await {
                                Ok(true) => link.send_message(Msg::CheckCamera(Ok(()))),
                                Ok(false) => link.send_message(Msg::CheckCamera(Err(BarcodeError {
                                    code: BarcodeErrorCode::CameraNotAvailable,
                                    message: "No camera available".to_string(),
                                }))),
                                Err(e) => link.send_message(Msg::CheckCamera(Err(e))),
                            }
                        }
                        Err(e) => link.send_message(Msg::CheckCamera(Err(e))),
                    }
                });
                
                true
            }
            Msg::CheckCamera(result) => {
                match result {
                    Ok(_) => {
                        if let Some(video) = self.video_ref.cast::<HtmlVideoElement>() {
                            match start_camera("barcode-video") {
                                Ok(_) => {
                                    self.state = State::CameraActive;
                                    // Start scanning task
                                    let link = ctx.link().clone();
                                    self.scan_task = Some(gloo_timers::future::TimeoutFuture::new(500).then(move |_| {
                                        link.send_message(Msg::BarcodeDetected(String::new()));
                                        async {}
                                    }));
                                }
                                Err(e) => {
                                    let error_str = e.as_string().unwrap_or_else(|| "Unknown camera error".into());
                                    let barcode_error = BarcodeError {
                                        code: BarcodeErrorCode::CameraError,
                                        message: error_str
                                    };
                                    ctx.props().on_error.emit(barcode_error.clone());
                                    self.state = State::Error(barcode_error);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        ctx.props().on_error.emit(e.clone());
                        match e.code {
                            BarcodeErrorCode::CameraPermissionDenied => {
                                self.state = State::PermissionDenied;
                            }
                            BarcodeErrorCode::CameraNotAvailable => {
                                self.state = State::CameraNotAvailable;
                            }
                            _ => {
                                self.state = State::Error(e);
                            }
                        }
                    }
                }
                true
            }
            Msg::BarcodeDetected(_barcode) => {
                if let (Some(video), Some(canvas)) = (
                    self.video_ref.cast::<HtmlVideoElement>(),
                    self.canvas_ref.cast::<HtmlCanvasElement>(),
                ) {
                    if let Some(barcode) = BarcodeService::scan_from_video(&video, &canvas) {
                        info!("Barcode detected: {}", barcode);
                        self.state = State::Scanning;
                        
                        // Dispatch to store
                        let dispatch = Dispatch::<ProductStore>::new();
                        dispatch.apply(crate::store::product::ProductAction::ScanBarcode(barcode.clone()));
                        
                        // Look up product by barcode using the barcode service
                        let future = async move {
                            match BarcodeService::scan_barcode(&barcode).await {
                                Ok(product) => Msg::ScanResult(Ok(product)),
                                Err(e) => Msg::ScanResult(Err(e)),
                            }
                        };
                        ctx.link().send_future(future);
                    } else {
                        // Schedule next scan
                        let link = ctx.link().clone();
                        self.scan_task = Some(gloo_timers::future::TimeoutFuture::new(500).then(move |_| {
                            link.send_message(Msg::BarcodeDetected(String::new()));
                            async {}
                        }));
                    }
                }
                true
            }
            Msg::ScanResult(result) => {
                match result {
                    Ok(product) => {
                        self.state = State::Success(product.clone());
                        ctx.props().on_product_found.emit(product);
                    }
                    Err(error) => {
                        self.state = State::Error(error.clone());
                        ctx.props().on_error.emit(error);
                    }
                }
                // Clear scanning task
                self.scan_task = None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.state {
            State::Idle => self.render_idle(ctx),
            State::CheckingCamera => self.render_checking(),
            State::PermissionDenied => self.render_permission_denied(),
            State::CameraNotAvailable => self.render_camera_unavailable(),
            State::CameraActive => self.render_camera(ctx),
            State::Scanning => self.render_scanning(ctx),
            State::Success(product) => self.render_success(ctx, product),
            State::Error(error) => self.render_error(ctx, error),
        }
    }
}

impl BarcodeScanner {
    fn render_idle(&self, ctx: &Context<Self>) -> Html {
        html! {
            <Button onclick={ctx.link().callback(|_| Msg::StartScan)}>
                {"Start Scanning"}
            </Button>
        }
    }
    
    fn render_checking(&self) -> Html {
        html! {
            <div class="scanning-overlay">
                <div class="scanning-indicator">
                    {"Checking camera..."}
                    <div class="spinner"></div>
                </div>
            </div>
        }
    }
    
    fn render_permission_denied(&self) -> Html {
        html! {
            <div class="scan-result error">
                <h3>{"Camera Permission Required"}</h3>
                <p>{"Please enable camera access in your browser settings"}</p>
            </div>
        }
    }
    
    fn render_camera_unavailable(&self) -> Html {
        html! {
            <div class="scan-result error">
                <h3>{"No Camera Available"}</h3>
                <p>{"This device doesn't have a camera or it's not accessible"}</p>
            </div>
        }
    }
    
    fn render_camera(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="barcode-scanner">
                <video
                    ref={self.video_ref.clone()}
                    id="barcode-video"
                    width="100%"
                    height="auto"
                    autoplay=true
                    playsinline=true
                />
                <canvas ref={self.canvas_ref.clone()} style="display: none;" />
            </div>
        }
    }
    
    fn render_scanning(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="scanning-overlay">
                <div class="scanning-indicator">
                    {"Scanning..."}
                    <div class="spinner"></div>
                </div>
            </div>
        }
    }
    
    fn render_success(&self, _ctx: &Context<Self>, product: &ProductResponse) -> Html {
        html! {
            <div class="scan-result success">
                <h3>{"Product Found!"}</h3>
                <p>{&product.name}</p>
                <p>{&product.brand}</p>
            </div>
        }
    }
    
    fn render_error(&self, ctx: &Context<Self>, error: &BarcodeError) -> Html {
        let message = match error.code {
            BarcodeErrorCode::InvalidBarcodeFormat => "Invalid barcode format",
            BarcodeErrorCode::ScanTimeout => "Scan timed out",
            _ => &error.message,
        };
        
        html! {
            <div class="scan-result error">
                <h3>{"Error"}</h3>
                <p>{message}</p>
                <Button onclick={ctx.link().callback(|_| Msg::StartScan)}>
                    {"Try Again"}
                </Button>
                <div class="manual-entry">
                    <p>{"or"}</p>
                    <Button onclick={ctx.link().callback(|_| Msg::StartScan)}>
                        {"Enter Manually"}
                    </Button>
                </div>
            </div>
        }
    }
}