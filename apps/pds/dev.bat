@echo off
REM Development script for CPC Desktop App on Windows
REM Supports hot reload for both frontend and backend

echo Starting CPC Desktop Development Environment...

REM Check if Node.js is installed
where node >nul 2>nul
if %errorlevel% neq 0 (
    echo Error: Node.js is not installed. Please install Node.js to continue.
    exit /b 1
)

REM Check if Rust is installed
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo Error: Rust is not installed. Please install Rust to continue.
    exit /b 1
)

REM Install frontend dependencies if needed
if not exist "frontend\node_modules" (
    echo Installing frontend dependencies...
    cd frontend && npm install && cd ..
)

REM Start development server
echo Starting Tauri development server...
cd src-tauri && cargo tauri dev