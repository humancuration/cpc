//! Base component trait for CPC web components
//!
//! This module defines the base component trait that all UI components
//! in the CPC ecosystem should implement.

use yew::prelude::*;

/// Base trait for all CPC UI components
///
/// This trait defines the common interface that all UI components
/// should implement to ensure consistency across the ecosystem.
///
/// ## Examples
///
/// ```
/// use web_core::components::{BaseComponent, CommonProps};
/// use yew::prelude::*;
///
/// struct MyComponent {
///     props: MyComponentProps,
/// }
///
/// #[derive(Properties, PartialEq, Clone)]
/// pub struct MyComponentProps {
///     #[prop_or_default]
///     pub common: CommonProps,
///     pub value: String,
/// }
///
/// impl BaseComponent for MyComponent {
///     type Properties = MyComponentProps;
///
///     fn create(props: &Self::Properties) -> Self {
///         Self { props: props.clone() }
///     }
///
///     fn update_props(&mut self, props: Self::Properties) {
///         self.props = props;
///     }
///
///     fn view(&self) -> Html {
///         html! {
///             <div class={self.props.common.class.clone()}>
///                 { &self.props.value }
///             </div>
///         }
///     }
/// }
/// ```
///
/// ## Related Modules
///
/// - [CommonProps]
/// - [ComponentSize]
/// - [ComponentTheme]
pub trait BaseComponent: Sized {
    /// Properties for the component
    type Properties: Properties;

    /// Create a new instance of the component
    ///
    /// ## Parameters
    ///
    /// - `props`: The properties to initialize the component with
    ///
    /// ## Returns
    ///
    /// A new instance of the component
    ///
    /// ## Examples
    ///
    /// ```
    /// use web_core::components::{BaseComponent, CommonProps};
    /// use yew::prelude::*;
    ///
    /// struct MyComponent {
    ///     props: MyComponentProps,
    /// }
    ///
    /// #[derive(Properties, PartialEq, Clone)]
    /// pub struct MyComponentProps {
    ///     #[prop_or_default]
    ///     pub common: CommonProps,
    ///     pub value: String,
    /// }
    ///
    /// impl BaseComponent for MyComponent {
    ///     type Properties = MyComponentProps;
    ///
    ///     fn create(props: &Self::Properties) -> Self {
    ///         Self { props: props.clone() }
    ///     }
    ///
    ///     // ... other methods
    /// #    fn update_props(&mut self, props: Self::Properties) { self.props = props; }
    /// #    fn view(&self) -> Html { html! {} }
    /// }
    ///
    /// let props = MyComponentProps {
    ///     common: CommonProps::default(),
    ///     value: "Hello, World!".to_string(),
    /// };
    /// let component = MyComponent::create(&props);
    /// ```
    fn create(props: &Self::Properties) -> Self;

    /// Update the component state based on new properties
    ///
    /// ## Parameters
    ///
    /// - `props`: The new properties to update the component with
    ///
    /// ## Examples
    ///
    /// ```
    /// use web_core::components::{BaseComponent, CommonProps};
    /// use yew::prelude::*;
    ///
    /// struct MyComponent {
    ///     props: MyComponentProps,
    /// }
    ///
    /// #[derive(Properties, PartialEq, Clone)]
    /// pub struct MyComponentProps {
    ///     #[prop_or_default]
    ///     pub common: CommonProps,
    ///     pub value: String,
    /// }
    ///
    /// impl BaseComponent for MyComponent {
    ///     type Properties = MyComponentProps;
    ///
    /// #    fn create(props: &Self::Properties) -> Self {
    /// #        Self { props: props.clone() }
    /// #    }
    /// #
    ///     fn update_props(&mut self, props: Self::Properties) {
    ///         self.props = props;
    ///     }
    /// #    fn view(&self) -> Html { html! {} }
    /// }
    ///
    /// let mut component = MyComponent {
    ///     props: MyComponentProps {
    ///         common: CommonProps::default(),
    ///         value: "Hello".to_string(),
    ///     },
    /// };
    ///
    /// let new_props = MyComponentProps {
    ///     common: CommonProps::default(),
    ///     value: "World".to_string(),
    /// };
    /// component.update_props(new_props);
    /// ```
    fn update_props(&mut self, props: Self::Properties);

    /// Render the component as HTML
    ///
    /// ## Returns
    ///
    /// The HTML representation of the component
    ///
    /// ## Examples
    ///
    /// ```
    /// use web_core::components::{BaseComponent, CommonProps};
    /// use yew::prelude::*;
    ///
    /// struct MyComponent {
    ///     props: MyComponentProps,
    /// }
    ///
    /// #[derive(Properties, PartialEq, Clone)]
    /// pub struct MyComponentProps {
    ///     #[prop_or_default]
    ///     pub common: CommonProps,
    ///     pub value: String,
    /// }
    ///
    /// impl BaseComponent for MyComponent {
    ///     type Properties = MyComponentProps;
    ///
    /// #    fn create(props: &Self::Properties) -> Self {
    /// #        Self { props: props.clone() }
    /// #    }
    /// #
    /// #    fn update_props(&mut self, props: Self::Properties) {
    /// #        self.props = props;
    /// #    }
    /// #
    ///     fn view(&self) -> Html {
    ///         html! {
    ///             <div class={self.props.common.class.clone()}>
    ///                 { &self.props.value }
    ///             </div>
    ///         }
    ///     }
    /// }
    /// ```
    fn view(&self) -> Html;
}

/// Common properties that all components should support
///
/// This struct provides a set of common properties that all UI components
/// should support to ensure consistency and ease of use across the ecosystem.
///
/// ## Examples
///
/// ```
/// use web_core::components::CommonProps;
///
/// let props = CommonProps {
///     class: Some("my-class".to_string()),
///     id: Some("my-id".to_string()),
///     disabled: false,
///     style: Some("color: red;".to_string()),
/// };
/// ```
///
/// ## Related Modules
///
/// - [BaseComponent]
#[derive(Properties, PartialEq, Clone)]
pub struct CommonProps {
    /// Additional CSS classes to apply
    ///
    /// These classes will be added to the component's root element.
    ///
    /// ## Examples
    ///
    /// ```
    /// use web_core::components::CommonProps;
    ///
    /// let props = CommonProps {
    ///     class: Some("btn btn-primary".to_string()),
    ///     ..Default::default()
    /// };
    /// ```
    #[prop_or_default]
    pub class: Option<String>,

    /// Unique identifier for the component
    ///
    /// This ID will be applied to the component's root element.
    ///
    /// ## Examples
    ///
    /// ```
    /// use web_core::components::CommonProps;
    ///
    /// let props = CommonProps {
    ///     id: Some("my-button".to_string()),
    ///     ..Default::default()
    /// };
    /// ```
    #[prop_or_default]
    pub id: Option<String>,

    /// Whether the component is disabled
    ///
    /// When true, the component should be rendered in a disabled state
    /// and should not respond to user interactions.
    ///
    /// ## Examples
    ///
    /// ```
    /// use web_core::components::CommonProps;
    ///
    /// let props = CommonProps {
    ///     disabled: true,
    ///     ..Default::default()
    /// };
    /// ```
    #[prop_or_default]
    pub disabled: bool,

    /// Custom styling
    ///
    /// Custom CSS styles to apply to the component's root element.
    ///
    /// ## Examples
    ///
    /// ```
    /// use web_core::components::CommonProps;
    ///
    /// let props = CommonProps {
    ///     style: Some("color: red; font-weight: bold;".to_string()),
    ///     ..Default::default()
    /// };
    /// ```
    #[prop_or_default]
    pub style: Option<String>,
}

/// Size variants for components
///
/// This enum defines the size variants that components can have.
///
/// ## Examples
///
/// ```
/// use web_core::components::ComponentSize;
///
/// let size = ComponentSize::Medium;
/// ```
///
/// ## Related Modules
///
/// - [BaseComponent]
/// - [CommonProps]
#[derive(PartialEq, Clone, Debug)]
pub enum ComponentSize {
    /// Small size variant
    ///
    /// Used for compact components that need to take up less space.
    Small,
    
    /// Medium size variant
    ///
    /// The default size for most components.
    Medium,
    
    /// Large size variant
    ///
    /// Used for components that need to be more prominent.
    Large,
}

impl Default for ComponentSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Theme variants for components
///
/// This enum defines the theme variants that components can have.
///
/// ## Examples
///
/// ```
/// use web_core::components::ComponentTheme;
///
/// let theme = ComponentTheme::Primary;
/// ```
///
/// ## Related Modules
///
/// - [BaseComponent]
/// - [CommonProps]
#[derive(PartialEq, Clone, Debug)]
pub enum ComponentTheme {
    /// Primary theme variant
    ///
    /// Used for primary actions and important elements.
    Primary,
    
    /// Secondary theme variant
    ///
    /// Used for secondary actions and less important elements.
    Secondary,
    
    /// Success theme variant
    ///
    /// Used for success messages and positive actions.
    Success,
    
    /// Warning theme variant
    ///
    /// Used for warning messages and cautionary actions.
    Warning,
    
    /// Danger theme variant
    ///
    /// Used for danger messages and destructive actions.
    Danger,
    
    /// Info theme variant
    ///
    /// Used for informational messages and neutral actions.
    Info,
    
    /// Light theme variant
    ///
    /// Used for light-themed elements.
    Light,
    
    /// Dark theme variant
    ///
    /// Used for dark-themed elements.
    Dark,
}

impl Default for ComponentTheme {
    fn default() -> Self {
        Self::Primary
    }
}