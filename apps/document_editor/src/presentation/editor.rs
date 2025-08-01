use yew::prelude::*;
use collaboration_engine::presence::UserPresence;
use collaboration_engine::core::Position;
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Properties, PartialEq)]
pub struct DocumentEditorProps {
    pub document_id: String,
}

#[function_component(DocumentEditor)]
pub fn document_editor(props: &DocumentEditorProps) -> Html {
    let document_content = use_state(|| String::from("Start typing your document here..."));
    let presences = use_state(|| Vec::<UserPresence>::new());
    
    let oninput = {
        let document_content = document_content.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            document_content.set(input.value());
        })
    };
    
    let view_presence_indicators = {
        let presences = presences.clone();
        move || {
            presences.iter().map(|presence| {
                if let Some(cursor) = &presence.cursor {
                    let color = match presence.qos_tier {
                        0 => "red",    // Critical QoS
                        1 => "orange", // Medium QoS
                        _ => "green",  // Low QoS
                    };
                    
                    html! {
                        <div
                            class="cursor-indicator"
                            style={format!("left: {}px; top: {}px; background-color: {}; position: absolute;",
                                         cursor.column * 10, cursor.line * 20, color)}
                        >
                            <span class="user-avatar" style="color: white; padding: 2px;">
                                {&presence.user_id.to_string()[..2]}
                            </span>
                            if presence.is_typing {
                                <span class="typing-indicator">{"|"}</span>
                            }
                        </div>
                    }
                } else {
                    html! {}
                }
            }).collect::<Vec<_>>()
        }
    };
    
    // Simulate updating presences - in a real implementation, this would come from the collaboration service
    {
        let presences = presences.clone();
        use_effect_with((), move |_| {
            // This is just for demonstration - in a real implementation, we would subscribe to presence updates
            presences.set(vec![
                UserPresence {
                    user_id: Uuid::new_v4(),
                    cursor: Some(Position { line: 0, column: 5 }),
                    selection: None,
                    last_activity: chrono::Utc::now(),
                    is_typing: true,
                    qos_tier: 0,
                }
            ]);
        });
    }
    
    html! {
        <div class="document-editor">
            <h1>{"Document Editor"}</h1>
            <div class="editor-container" style="position: relative;">
                <textarea
                    class="editor-textarea"
                    value={(*document_content).clone()}
                    oninput={oninput}
                    rows="20"
                    cols="80"
                />
                // Presence indicators
                {view_presence_indicators()}
            </div>
            <div class="editor-actions">
                <button>{"Save"}</button>
                <button>{"Export"}</button>
            </div>
        </div>
    }
}