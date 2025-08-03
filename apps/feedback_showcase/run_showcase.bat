@echo off
:: Feedback System Showcase Runner for Windows

title Feedback System Showcase

echo === Feedback System Showcase ===
echo This script will run the showcase and clean up generated files
echo.

:: Run the showcase
echo Running the showcase...
cargo run

:: Check if the command succeeded
if %ERRORLEVEL% EQU 0 (
    echo.
    echo Showcase completed successfully!
    echo.
    
    :: List generated files
    echo Generated files:
    dir *.png *.svg *.html 2>nul || echo No visualization files found
    echo.
    
    :: Offer to clean up
    set /p CLEANUP=Do you want to clean up generated files? (y/n): 
    if /i "%CLEANUP%"=="y" (
        echo Cleaning up generated files...
        del /q rating_distribution.png trend_comparison.svg correlation_matrix.html 2>nul
        echo Cleanup complete!
    ) else (
        echo Generated files kept.
    )
) else (
    echo Showcase failed to run!
    pause
    exit /b 1
)

pause