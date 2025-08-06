# Authentication

The web_core authentication module provides services for user authentication and session management.

## AuthService

The `AuthService` struct provides methods for user authentication:

```rust
use web_core::auth::{AuthService, User, Token};

let auth_service = AuthService::new();

// Login a user
// let user = auth_service.login("username", "password").await?;

// Get the current user
if let Some(user) = auth_service.get_current_user() {
    // User is logged in
    println!("User: {} ({})", user.username, user.email);
}

// Get the current JWT token
if let Some(token) = auth_service.get_token() {
    // Token is available
    println!("Token expires in: {} seconds", token.expires_in);
}

// Logout the current user
// auth_service.logout().await?;
```

## User Model

The `User` struct represents a user in the system:

```rust
use web_core::auth::User;
use uuid::Uuid;

let user = User {
    id: Uuid::new_v4(),
    username: "john_doe".to_string(),
    email: "john@example.com".to_string(),
};
```

## Token Model

The `Token` struct represents a JWT token:

```rust
use web_core::auth::Token;

let token = Token {
    access_token: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...".to_string(),
    refresh_token: Some("refresh_token_here".to_string()),
    expires_in: 3600, // 1 hour
};
```

## Session Management

The auth service automatically manages user sessions using browser storage:

```rust
use web_core::auth::AuthService;

let auth_service = AuthService::new();

// Save user to session storage
// auth_service.save_current_user(&user)?;

// Clear user from session storage
// auth_service.clear_current_user()?;

// Save token to session storage
// auth_service.save_token(&token)?;

// Clear token from session storage
// auth_service.clear_token()?;
```

## Login Flow

A typical login flow using the auth service:

```rust
use web_core::auth::AuthService;
use web_core::utils::error_handling::WebError;

async fn login(username: &str, password: &str) -> Result<(), WebError> {
    let auth_service = AuthService::new();
    
    match auth_service.login(username, password).await {
        Ok(user) => {
            // Login successful
            println!("Welcome, {}!", user.username);
            Ok(())
        }
        Err(error) => {
            // Login failed
            Err(WebError::AuthenticationError(error))
        }
    }
}

async fn logout() -> Result<(), WebError> {
    let auth_service = AuthService::new();
    
    match auth_service.logout().await {
        Ok(()) => {
            // Logout successful
            println!("You have been logged out");
            Ok(())
        }
        Err(error) => {
            // Logout failed
            Err(WebError::AuthenticationError(error))
        }
    }
}
```

## Protected Routes

Check authentication status for protected routes:

```rust
use web_core::auth::AuthService;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/dashboard")]
    Dashboard,
}

fn switch/routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Login => html! { <Login /> },
        Route::Dashboard => {
            // Check if user is authenticated
            let auth_service = AuthService::new();
            if auth_service.get_current_user().is_some() {
                html! { <Dashboard /> }
            } else {
                // Redirect to login
                html! { <Redirect<Route> to={Route::Login} /> }
            }
        }
    }
}
```

## Token Refresh

Handle token refresh for expired sessions:

```rust
use web_core::auth::{AuthService, Token};
use web_core::utils::error_handling::WebError;

async fn refresh_token() -> Result<(), WebError> {
    let auth_service = AuthService::new();
    
    // Check if we have a refresh token
    if let Some(token) = auth_service.get_token() {
        if let Some(refresh_token) = token.refresh_token {
            // Call the refresh endpoint
            // let new_token = call_refresh_api(&refresh_token).await?;
            
            // Save the new token
            // auth_service.save_token(&new_token)?;
            
            // Ok(())
        }
    }
    
    Err(WebError::AuthenticationError("No refresh token available".to_string()))
}
```

## Best Practices

1. **Always validate tokens** - Check token expiration and validity before making API calls.

2. **Handle authentication errors gracefully** - Redirect users to login when authentication fails.

3. **Securely store tokens** - Use secure storage mechanisms and avoid storing tokens in plain text.

4. **Implement automatic token refresh** - Refresh tokens before they expire to maintain user sessions.

5. **Clear session data on logout** - Ensure all user data is cleared when logging out.

6. **Use HTTPS in production** - Always use HTTPS to protect authentication data in transit.

7. **Implement proper error handling** - Handle authentication errors with appropriate user feedback.

8. **Log security events** - Log authentication events for security monitoring and auditing.

9. **Implement rate limiting** - Prevent brute force attacks with rate limiting on authentication endpoints.

10. **Use strong password policies** - Enforce strong passwords and implement password strength validation.