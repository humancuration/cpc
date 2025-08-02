use yew::prelude::*;

mod app;
mod components;
mod grpc;
mod services;

fn main() {
    yew::Renderer::<app::App>::new().render();
}