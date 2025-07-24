import { writable } from 'svelte/store';

// Auth token store
export const authToken = writable(localStorage.getItem('authToken') || '');

// Current user store
export const currentUser = writable(null);

// Subscribe to auth token changes
authToken.subscribe(token => {
  if (token) {
    localStorage.setItem('authToken', token);
  } else {
    localStorage.removeItem('authToken');
  }
});

// Login function
export async function login(username, password) {
  try {
    const response = await fetch('http://localhost:3000/api/auth/login', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ username, password }),
    });
    
    if (!response.ok) {
      throw new Error('Login failed');
    }
    
    const data = await response.json();
    authToken.set(data.token);
    currentUser.set(data.user);
    
    return data;
  } catch (error) {
    throw new Error(error.message || 'Login failed');
  }
}

// Logout function
export function logout() {
  authToken.set(null);
  currentUser.set(null);
}

// Check auth status on app load
export async function checkAuthStatus() {
  const token = localStorage.getItem('authToken');
  if (!token) return;
  
  try {
    const response = await fetch('http://localhost:3000/api/auth/me', {
      headers: {
        'Authorization': `Bearer ${token}`,
      },
    });
    
    if (response.ok) {
      const user = await response.json();
      currentUser.set(user);
    } else {
      // Token is invalid, clear it
      logout();
    }
  } catch (error) {
    console.error('Auth check failed:', error);
    logout();
  }
}