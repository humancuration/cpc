//! WebSocket service for the Messenger web application

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket};
use yew::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

/// Events that can be sent over WebSocket
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketEvent {
    /// A reaction was added to a message
    ReactionAdded {
        message_id: Uuid,
        reaction: crate::models::Reaction,
    },
    /// A reaction was removed from a message
    ReactionRemoved {
        message_id: Uuid,
        reaction_id: Uuid,
    },
    /// A message was updated
    MessageUpdated {
        message: crate::models::Message,
    },
    /// A message was deleted
    MessageDeleted {
        message_id: Uuid,
    },
    /// Connection acknowledged
    Connected {
        user_id: Uuid,
    },
}

/// Service for handling WebSocket connections
pub struct WebSocketService {
    websocket: Option<WebSocket>,
    on_message_callback: Option<Box<dyn Fn(WebSocketEvent) -> Result<(), String>>>,
}

impl WebSocketService {
    /// Create a new WebSocket service
    pub fn new() -> Self {
        Self {
            websocket: None,
            on_message_callback: None,
        }
    }
    
    /// Connect to the WebSocket server
    pub fn connect(&mut self, url: &str) -> Result<(), String> {
        // Create WebSocket connection
        let ws = WebSocket::new(url)
            .map_err(|_| "Failed to create WebSocket connection".to_string())?;
        
        // Clone the WebSocket for use in callbacks
        let ws_clone = ws.clone();
        
        // Set up message handler
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                let text = txt.as_string().unwrap_or_default();
                // In a real implementation, we would parse the message and call the callback
                // For now, we'll just log it
                web_sys::console::log_1(&format!("Received message: {}", text).into());
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
        
        // Set up error handler
        let onerror_callback = Closure::wrap(Box::new(move |_e: web_sys::Event| {
            web_sys::console::error_1(&"WebSocket error".into());
        }) as Box<dyn FnMut(web_sys::Event)>);
        
        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();
        
        // Set up close handler
        let onclose_callback = Closure::wrap(Box::new(move |_e: web_sys::CloseEvent| {
            web_sys::console::log_1(&"WebSocket connection closed".into());
        }) as Box<dyn FnMut(web_sys::CloseEvent)>);
        
        ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
        onclose_callback.forget();
        
        // Set up open handler
        let onopen_callback = Closure::wrap(Box::new(move |_e: web_sys::Event| {
            web_sys::console::log_1(&"WebSocket connection opened".into());
        }) as Box<dyn FnMut(web_sys::Event)>);
        
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
        
        self.websocket = Some(ws);
        Ok(())
    }
    
    /// Send a message over the WebSocket
    pub fn send_message(&self, event: &WebSocketEvent) -> Result<(), String> {
        if let Some(ws) = &self.websocket {
            let json = serde_json::to_string(event)
                .map_err(|e| format!("Failed to serialize message: {}", e))?;
            
            ws.send_with_str(&json)
                .map_err(|_| "Failed to send message".to_string())
        } else {
            Err("WebSocket not connected".to_string())
        }
    }
    
    /// Close the WebSocket connection
    pub fn close(&mut self) {
        if let Some(ws) = self.websocket.take() {
            let _ = ws.close();
        }
    }
    
    /// Set the callback for handling incoming messages
    pub fn set_on_message(&mut self, callback: Box<dyn Fn(WebSocketEvent) -> Result<(), String>>) {
        self.on_message_callback = Some(callback);
    }
}

impl Default for WebSocketService {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for WebSocketService {
    fn drop(&mut self) {
        self.close();
    }
}