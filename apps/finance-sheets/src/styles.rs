//! Styling utilities for Finance-Sheets
//!
//! This module contains utility functions and constants for styling the application,
//! with a focus on responsive design and mobile optimization.

use stylist::{Style, style};
use yew::prelude::*;

/// Get the base mobile styles for the application
/// 
/// These styles implement the mobile-first approach as outlined in the mobile
/// optimization plan, with appropriate breakpoints and touch-friendly sizing.
pub fn get_mobile_styles() -> Style {
    style! {
        r#"
        /* Mobile-first base styles */
        .finance-sheets-app {
            font-size: 16px;
            line-height: 1.5;
            min-height: 100vh;
            display: flex;
            flex-direction: column;
        }
        
        /* Touch target optimization - minimum 48px */
        .touch-target {
            min-height: 48px;
            min-width: 48px;
            padding: 12px;
            display: flex;
            align-items: center;
            justify-content: center;
        }
        
        /* Mobile layout components */
        .mobile-layout {
            display: flex;
            flex-direction: row;
            min-height: 100vh;
        }
        
        .navigation-rail {
            width: 72px;
            background-color: #f5f5f5;
            display: flex;
            flex-direction: column;
            padding: 12px 0;
        }
        
        .navigation-rail ul {
            list-style: none;
            padding: 0;
            margin: 0;
        }
        
        .nav-button {
            background: none;
            border: none;
            padding: 16px 12px;
            width: 100%;
            text-align: center;
            font-size: 14px;
            cursor: pointer;
        }
        
        .mobile-content {
            flex: 1;
            padding: 16px;
            overflow-y: auto;
        }
        
        .bottom-navigation {
            position: fixed;
            bottom: 0;
            left: 0;
            right: 0;
            height: 56px;
            background-color: #ffffff;
            box-shadow: 0 -2px 4px rgba(0,0,0,0.1);
            display: flex;
            align-items: center;
            justify-content: space-around;
            z-index: 100;
        }
        
        .bottom-navigation ul {
            list-style: none;
            display: flex;
            width: 100%;
            padding: 0;
            margin: 0;
        }
        
        .bottom-nav-button {
            flex: 1;
            background: none;
            border: none;
            padding: 8px 0;
            text-align: center;
            font-size: 12px;
            cursor: pointer;
        }
        
        .fab {
            position: fixed;
            bottom: 72px;
            right: 16px;
            width: 56px;
            height: 56px;
            border-radius: 50%;
            background-color: #007bff;
            color: white;
            border: none;
            font-size: 24px;
            box-shadow: 0 2px 8px rgba(0,0,0,0.2);
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
        }
        
        /* Responsive breakpoints */
        @media (max-width: 768px) {
            .tool-section {
                margin-bottom: 24px;
            }
            
            .currency-tools {
                display: flex;
                flex-direction: column;
                gap: 16px;
            }
            
            h1 {
                font-size: 1.5rem;
                margin: 0;
                padding: 16px;
            }
            
            h2 {
                font-size: 1.25rem;
            }
        }
        
        @media (min-width: 769px) and (max-width: 1024px) {
            .mobile-layout {
                display: none;
            }
        }
        
        @media (min-width: 1025px) {
            .mobile-layout {
                display: none;
            }
        }
        "#
    }.expect("Failed to create mobile styles")
}

/// Get responsive styles for mobile devices
pub fn get_responsive_styles() -> Style {
    style! {
        r#"
        /* Base responsive styles */
        .responsive-container {
            width: 100%;
            max-width: 100%;
            padding: 0 16px;
            box-sizing: border-box;
        }
        
        /* Fluid typography */
        .fluid-text {
            font-size: clamp(1rem, 2.5vw, 1.25rem);
        }
        
        /* Flexible grid system */
        .grid {
            display: grid;
            gap: 16px;
            grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
        }
        
        /* Mobile-friendly form elements */
        .mobile-input {
            width: 100%;
            padding: 16px;
            font-size: 16px; /* Prevents zoom on iOS */
            border: 1px solid #ddd;
            border-radius: 4px;
            box-sizing: border-box;
        }
        
        .mobile-select {
            width: 100%;
            padding: 16px;
            font-size: 16px;
            border: 1px solid #ddd;
            border-radius: 4px;
            background-color: white;
            box-sizing: border-box;
        }
        
        /* Touch-friendly buttons */
        .mobile-button {
            width: 100%;
            padding: 16px;
            font-size: 16px;
            border: none;
            border-radius: 4px;
            cursor: pointer;
            text-align: center;
        }
        "#
    }.expect("Failed to create responsive styles")
}