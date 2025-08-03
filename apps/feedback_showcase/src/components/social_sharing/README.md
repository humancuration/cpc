# Social Sharing Components

This module provides components for sharing visualizations through various channels including federation, social media, embedding, and image export.

## Components

### ShareButtonGroup
The main component that provides sharing options for visualizations. It includes buttons for:
- Federation sharing (🌐)
- Embed code generation (</>)
- Image export (📷)
- Social media sharing (🔗)

### SocialSharingDialog
A dialog component that allows users to share visualizations to social media platforms (Twitter, Facebook, LinkedIn). It includes:
- Platform selection
- Message customization
- Preview of shared content
- Alt text display

### EmbedCodeDialog
A dialog component for customizing and generating embed codes for visualizations. Features include:
- Width/height customization
- Theme selection (light/dark)
- Title visibility toggle
- Interactivity toggle
- Live preview
- Copy to clipboard functionality

### EmbedPreview
A component that provides a preview of how an embedded visualization will look with the current settings.

### AnnotationManager
A component for managing annotations on embedded visualizations. It allows users to:
- Add new annotations
- View existing annotations
- See annotation metadata (user, timestamp)

### EmbeddedVisualization
A component for displaying embedded visualizations with annotation support.

## Services

### Federation Service
Handles sharing visualizations to the federation network using p2panda. Includes:
- `share_visualization` - Shares visualization data to the federation
- `get_shared_visualization` - Retrieves shared visualization data
- `generate_embed_code` - Generates embed code for shared visualizations

### Embed Code Generator
Utilities for generating embed codes:
- `generate_embed_code` - Generates basic embed code
- `generate_markdown_embed` - Generates markdown embed code
- `generate_custom_embed_code` - Generates embed code with customization options

### Image Exporter
Utilities for exporting visualizations as images:
- `export_as_image` - Exports canvas as image file
- `export_as_image_with_options` - Exports canvas with custom format and quality
- `get_image_data_base64` - Gets image data as base64 string

## Accessibility

All social sharing components follow WCAG 2.1 AA compliance guidelines:
- Proper ARIA attributes
- Keyboard navigation support
- Color contrast checks
- Screen reader compatibility

## Usage

To use the social sharing components in a visualization:

```rust
use yew::prelude::*;
use crate::components::social_sharing::ShareButtonGroup;
use crate::components::visualization::types::{VisualizationProps, VisualizationComponent};

#[function_component(MyVisualization)]
pub fn my_visualization(props: &VisualizationProps) -> Html {
    let canvas_ref = use_node_ref();
    
    html! {
        <div class="my-visualization">
            <div class="visualization-header">
                <h2>{"My Visualization"}</h2>
                <ShareButtonGroup
                    visualization_type={VisualizationComponent::Summary}
                    reviews={props.reviews.clone()}
                    canvas_ref={canvas_ref.clone()}
                    on_share={props.on_share.clone()}
                />
            </div>
            <canvas ref={canvas_ref} />
        </div>
    }
}
```

## Customization

The embed code dialog allows users to customize:
- Width (200-2000px)
- Height (100-1500px)
- Theme (light/dark)
- Title visibility
- Interactivity

## Testing

Unit tests are included for embed code generation functions in `social_sharing_test.rs`.