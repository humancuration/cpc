//! Mobile-optimized spreadsheet grid component
//!
//! This component provides a touch-friendly spreadsheet grid implementation
//! optimized for mobile devices with larger touch targets and gesture support.

use yew::prelude::*;
use stylist::yew::use_style;
use stylist::Style;
use crate::services::mobile::performance::VirtualScrollManager;

/// Properties for the mobile sheet grid component
#[derive(Properties, PartialEq)]
pub struct MobileSheetGridProps {
    /// Number of rows in the grid
    pub rows: usize,
    
    /// Number of columns in the grid
    pub cols: usize,
    
    /// Callback when a cell is selected
    #[prop_or_default]
    pub on_cell_select: Callback<(usize, usize)>,
    
    /// Callback when a cell value changes
    #[prop_or_default]
    pub on_cell_change: Callback<(usize, usize, String)>,
    
    /// Callback for swipe navigation
    #[prop_or_default]
    pub on_swipe: Callback<SwipeDirection>,
}

/// Swipe directions for gesture handling
#[derive(Debug, Clone, PartialEq)]
pub enum SwipeDirection {
    Up,
    Down,
    Left,
    Right,
}

/// State for the mobile sheet grid component
#[derive(Debug, Clone, PartialEq)]
pub struct MobileSheetGridState {
    /// Currently selected cell (row, col)
    selected_cell: Option<(usize, usize)>,
    
    /// Cell values
    cell_values: Vec<Vec<String>>,
    
    /// Touch start position for swipe detection
    touch_start: Option<(i32, i32)>,
    
    /// Virtual scroll manager for performance
    scroll_manager: VirtualScrollManager,
}

/// Messages for the mobile sheet grid component
#[derive(Debug, Clone)]
pub enum MobileSheetGridMsg {
    /// Select a cell
    SelectCell(usize, usize),
    
    /// Change cell value
    ChangeCellValue(usize, usize, String),
    
    /// Handle touch start event
    TouchStart(i32, i32),
    
    /// Handle touch end event
    TouchEnd(i32, i32),
    
    /// Scroll the grid
    Scroll(f64),
}

/// Mobile-optimized spreadsheet grid component
#[derive(Debug)]
pub struct MobileSheetGrid {
    /// Component state
    state: MobileSheetGridState,
}

impl Component for MobileSheetGrid {
    type Message = MobileSheetGridMsg;
    type Properties = MobileSheetGridProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        
        // Initialize cell values
        let mut cell_values = Vec::with_capacity(props.rows);
        for _ in 0..props.rows {
            let mut row = Vec::with_capacity(props.cols);
            for _ in 0..props.cols {
                row.push(String::new());
            }
            cell_values.push(row);
        }
        
        Self {
            state: MobileSheetGridState {
                selected_cell: None,
                cell_values,
                touch_start: None,
                scroll_manager: VirtualScrollManager::new(
                    props.rows,
                    20, // Render 20 rows at a time
                    48.0, // 48px cell height for touch targets
                ),
            },
        }
    }
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MobileSheetGridMsg::SelectCell(row, col) => {
                self.state.selected_cell = Some((row, col));
                ctx.props().on_cell_select.emit((row, col));
                true
            }
            
            MobileSheetGridMsg::ChangeCellValue(row, col, value) => {
                if row < self.state.cell_values.len() && col < self.state.cell_values[row].len() {
                    self.state.cell_values[row][col] = value.clone();
                    ctx.props().on_cell_change.emit((row, col, value));
                }
                true
            }
            
            MobileSheetGridMsg::TouchStart(x, y) => {
                self.state.touch_start = Some((x, y));
                true
            }
            
            MobileSheetGridMsg::TouchEnd(x, y) => {
                if let Some((start_x, start_y)) = self.state.touch_start {
                    let delta_x = x - start_x;
                    let delta_y = y - start_y;
                    
                    // Minimum swipe distance
                    const MIN_SWIPE_DISTANCE: i32 = 50;
                    
                    // Determine swipe direction
                    if delta_x.abs() > MIN_SWIPE_DISTANCE || delta_y.abs() > MIN_SWIPE_DISTANCE {
                        let direction = if delta_x.abs() > delta_y.abs() {
                            if delta_x > 0 {
                                SwipeDirection::Right
                            } else {
                                SwipeDirection::Left
                            }
                        } else {
                            if delta_y > 0 {
                                SwipeDirection::Down
                            } else {
                                SwipeDirection::Up
                            }
                        };
                        
                        ctx.props().on_swipe.emit(direction);
                    }
                }
                
                self.state.touch_start = None;
                true
            }
            
            MobileSheetGridMsg::Scroll(position) => {
                self.state.scroll_manager.update_scroll_position(position);
                true
            }
        }
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        let style = get_mobile_sheet_styles();
        let props = ctx.props();
        let link = ctx.link();
        
        // Get render range from virtual scroll manager
        let (start_row, end_row) = self.state.scroll_manager.get_render_range();
        
        // Touch event handlers
        let on_touch_start = link.callback(|e: TouchEvent| {
            if let Some(touch) = e.touches().get(0) {
                MobileSheetGridMsg::TouchStart(touch.client_x(), touch.client_y())
            } else {
                // Fallback if we can't get touch data
                MobileSheetGridMsg::TouchStart(0, 0)
            }
        });
        
        let on_touch_end = link.callback(|e: TouchEvent| {
            if let Some(touch) = e.changed_touches().get(0) {
                MobileSheetGridMsg::TouchEnd(touch.client_x(), touch.client_y())
            } else {
                // Fallback if we can't get touch data
                MobileSheetGridMsg::TouchEnd(0, 0)
            }
        });
        
        // Scroll handler
        let on_scroll = link.callback(|e: Event| {
            let target: web_sys::HtmlElement = e.target_unchecked_into();
            MobileSheetGridMsg::Scroll(target.scroll_top() as f64)
        });
        
        html! {
            <div 
                class={style}
                ontouchstart={on_touch_start}
                ontouchend={on_touch_end}
            >
                <div 
                    class="mobile-sheet-container"
                    onscroll={on_scroll}
                >
                    <div 
                        class="mobile-sheet-content"
                        style={format!("height: {}px", self.state.scroll_manager.get_total_height())}
                    >
                        <div 
                            class="mobile-sheet-offset"
                            style={format!("transform: translateY({}px)", self.state.scroll_manager.get_offset())}
                        >
                            <table class="mobile-sheet-grid">
                                <tbody>
                                    {for (start_row..end_row).map(|row_idx| {
                                        html! {
                                            <tr key={row_idx}>
                                                {for (0..props.cols).map(|col_idx| {
                                                    let is_selected = self.state.selected_cell == Some((row_idx, col_idx));
                                                    let value = if row_idx < self.state.cell_values.len() && col_idx < self.state.cell_values[row_idx].len() {
                                                        self.state.cell_values[row_idx][col_idx].clone()
                                                    } else {
                                                        String::new()
                                                    };
                                                    
                                                    let on_cell_click = link.callback(move |_| {
                                                        MobileSheetGridMsg::SelectCell(row_idx, col_idx)
                                                    });
                                                    
                                                    let on_input_change = link.callback(move |e: InputEvent| {
                                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                        MobileSheetGridMsg::ChangeCellValue(row_idx, col_idx, input.value())
                                                    });
                                                    
                                                    html! {
                                                        <td 
                                                            class={classes!("mobile-sheet-cell", is_selected.then(|| "selected"))}
                                                            onclick={on_cell_click}
                                                        >
                                                            <input
                                                                type="text"
                                                                class="mobile-cell-input"
                                                                value={value}
                                                                oninput={on_input_change}
                                                                placeholder=""
                                                            />
                                                        </td>
                                                    }
                                                })}
                                            </tr>
                                        }
                                    })}
                                </tbody>
                            </table>
                        </div>
                    </div>
                </div>
                
                if let Some((row, col)) = self.state.selected_cell {
                    <div class="mobile-cell-toolbar">
                        <button class="toolbar-button">{"Bold"}</button>
                        <button class="toolbar-button">{"Italic"}</button>
                        <button class="toolbar-button">{"Currency"}</button>
                        <button class="toolbar-button">{"Format"}</button>
                    </div>
                }
            </div>
        }
    }
}

/// Get the CSS styles for the mobile sheet grid
fn get_mobile_sheet_styles() -> Style {
    use_style!(
        r#"
        .mobile-sheet-container {
            width: 100%;
            height: calc(100vh - 120px);
            overflow: auto;
            position: relative;
        }
        
        .mobile-sheet-content {
            position: relative;
        }
        
        .mobile-sheet-offset {
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
        }
        
        .mobile-sheet-grid {
            width: 100%;
            border-collapse: collapse;
            table-layout: fixed;
        }
        
        .mobile-sheet-cell {
            border: 1px solid #ddd;
            padding: 0;
            height: 48px; /* Minimum touch target size */
            min-width: 80px;
            position: relative;
        }
        
        .mobile-sheet-cell.selected {
            border: 2px solid #007bff;
            background-color: #e6f0ff;
        }
        
        .mobile-cell-input {
            width: 100%;
            height: 100%;
            border: none;
            padding: 8px;
            font-size: 16px; /* Prevents zoom on iOS */
            box-sizing: border-box;
            background: transparent;
        }
        
        .mobile-cell-input:focus {
            outline: none;
        }
        
        .mobile-cell-toolbar {
            position: fixed;
            bottom: 72px; /* Above FAB */
            left: 0;
            right: 0;
            background-color: white;
            border-top: 1px solid #ddd;
            padding: 8px;
            display: flex;
            justify-content: space-around;
        }
        
        .toolbar-button {
            flex: 1;
            padding: 12px;
            margin: 0 4px;
            border: 1px solid #ddd;
            border-radius: 4px;
            background-color: white;
            font-size: 14px;
            min-height: 48px; /* Touch target optimization */
        }
        
        .toolbar-button:active {
            background-color: #f0f0f0;
        }
    "#
    )
}