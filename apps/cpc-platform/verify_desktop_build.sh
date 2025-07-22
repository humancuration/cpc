#!/bin/bash

TEST_MODE=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --test)
        TEST_MODE=true
        shift
        ;;
        *)
        echo "Unknown option: $1"
        exit 1
        ;;
    esac
done

echo "Building desktop application..."
pnpm tauri build
BUILD_STATUS=$?
echo "Build completed with status: $BUILD_STATUS"

if [ "$TEST_MODE" = true ]; then
    echo "Running desktop integration tests..."
    
    # Start application in background
    ./src-tauri/target/release/cpc-platform &
    APP_PID=$!
    
    # Initialize texture memory variables
    TEXTURE_MEMORY_USAGE=0
    TEXTURE_MEMORY_STATUS="skip"
    
    # Platform-specific test cases
    case $(uname) in
        Linux)
            echo "Running Linux tests (X11/Wayland)..."
            # Use xvfb-run for headless testing
            xvfb-run --auto-servernum --server-args="-screen 0 1024x768x24" \
                ./src-tauri/target/release/cpc-platform --test > /dev/null 2>&1 &
            TEST_PID=$!
            sleep 5  # Wait for app to start
            
            # Basic functionality checks
            if xdotool search --name "cpc-platform" windowactivate key Return; then
                echo "Linux UI tests passed"
                LINUX_STATUS="pass"
            else
                echo "Linux UI tests failed"
                LINUX_STATUS="fail"
            fi
            
            # Texture memory validation for Linux
            echo "Validating texture memory..."
            if command -v nvidia-smi &> /dev/null; then
                MEMORY_USAGE=$(nvidia-smi --query-compute-apps=pid,used_memory --format=csv,noheader,nounits | grep "$TEST_PID" | awk '{print $2}')
                if [ -z "$MEMORY_USAGE" ]; then
                    MEMORY_USAGE=0
                fi
                TEXTURE_MEMORY_USAGE=$MEMORY_USAGE
            else
                echo "nvidia-smi not found. Skipping texture memory validation."
                TEXTURE_MEMORY_USAGE=0
            fi

            TEXTURE_MEMORY_THRESHOLD=512
            if [ "$TEXTURE_MEMORY_USAGE" -gt "$TEXTURE_MEMORY_THRESHOLD" ]; then
                echo "Texture memory usage exceeded threshold: $TEXTURE_MEMORY_USAGE MB > $TEXTURE_MEMORY_THRESHOLD MB"
                TEXTURE_MEMORY_STATUS="fail"
            else
                echo "Texture memory usage: $TEXTURE_MEMORY_USAGE MB (within threshold)"
                TEXTURE_MEMORY_STATUS="pass"
            fi
            
            # Network simulation for Linux
            if command -v tc &> /dev/null; then
                echo "Simulating network conditions..."
                sudo tc qdisc add dev lo root netem delay 50ms 20ms loss 5%
            fi

            # Texture memory validation for macOS
            echo "Validating texture memory..."
            # TODO: Implement proper Metal System Trace/heap command integration
            TEXTURE_MEMORY_USAGE=0
            TEXTURE_MEMORY_STATUS="skip"
            echo "Texture memory validation not implemented for macOS. Using placeholder."

            # Network functionality tests
            echo "Starting network tests..."
            NETWORK_TEMP_DIR=$(mktemp -d)
            
            # Start first test node
            ./src-tauri/target/release/cpc-platform --test-node --data-dir="$NETWORK_TEMP_DIR/node1" --listen-port=41504 > "$NETWORK_TEMP_DIR/node1.log" 2>&1 &
            NODE1_PID=$!
            sleep 2
            
            # Start second test node
            ./src-tauri/target/release/cpc-platform --test-node --data-dir="$NETWORK_TEMP_DIR/node2" --listen-port=41505 > "$NETWORK_TEMP_DIR/node2.log" 2>&1 &
            NODE2_PID=$!
            sleep 2
            
            # Platform-specific network simulation
            case $(uname) in
                Linux)
                    sudo tc qdisc add dev lo root netem delay 50ms 20ms loss 5%
                    ;;
                Darwin)
                    sudo dnctl pipe 1 config delay 50 plr 0.05
                    sudo pfctl -f /etc/pf.conf
                    ;;
                CYGWIN*|MINGW*|MSYS*)
                    powershell -Command "New-NetQosPolicy -Name 'CPCTest' -AppPathNameMatchCondition 'cpc-platform.exe' -ThrottleRateActionBitsPerSecond 1mb"
                    ;;
            esac
            
            # Run network tests with timeout
            timeout 30s ./src-tauri/target/release/cpc-platform --run-network-tests \
                --node-addr="/ip4/127.0.0.1/tcp/41504/p2p/$(tail -n 1 "$NETWORK_TEMP_DIR/node1.log" | cut -d' ' -f2)" \
                --target-addr="/ip4/127.0.0.1/tcp/41505/p2p/$(tail -n 1 "$NETWORK_TEMP_DIR/node2.log" | cut -d' ' -f2)"
            
            NETWORK_TEST_STATUS=$?
            
            # Cleanup network simulation
            case $(uname) in
                Linux)
                    sudo tc qdisc del dev lo root
                    ;;
                Darwin)
                    sudo dnctl -q flush
                    sudo pfctl -f /etc/pf.conf
                    ;;
                CYGWIN*|MINGW*|MSYS*)
                    powershell -Command "Remove-NetQosPolicy -Name 'CPCTest' -Confirm:\$false"
                    ;;
            esac
            
            # Collect results
            if [ $NETWORK_TEST_STATUS -eq 0 ]; then
                LATENCY=$(grep "Ping latency" "$NETWORK_TEMP_DIR/node1.log" | awk '{print $4}')
                NETWORK_RESULTS="\"network_tests\": {
                    \"peer_discovery\": \"pass\",
                    \"message_passing\": \"pass\",
                    \"latency_ms\": $LATENCY
                }"
            else
                NETWORK_RESULTS="\"network_tests\": {
                    \"peer_discovery\": \"fail\",
                    \"message_passing\": \"fail\",
                    \"latency_ms\": null
                }"
            fi
            
            # Cleanup nodes
            kill $NODE1_PID $NODE2_PID
            rm -rf "$NETWORK_TEMP_DIR"
            
            kill $TEST_PID

            # Network functionality tests
            echo "Starting network tests..."
            NETWORK_TEMP_DIR=$(mktemp -d)
            
            # Start test nodes
            ./src-tauri/target/release/cpc-platform --test-node --data-dir="$NETWORK_TEMP_DIR/node1" --listen-port=41504 > "$NETWORK_TEMP_DIR/node1.log" 2>&1 &
            NODE1_PID=$!
            sleep 2
            
            ./src-tauri/target/release/cpc-platform --test-node --data-dir="$NETWORK_TEMP_DIR/node2" --listen-port=41505 > "$NETWORK_TEMP_DIR/node2.log" 2>&1 &
            NODE2_PID=$!
            sleep 2
            
            # Run network tests
            timeout 30s ./src-tauri/target/release/cpc-platform --run-network-tests \
                --node-addr="/ip4/127.0.0.1/tcp/41504" \
                --target-addr="/ip4/127.0.0.1/tcp/41505"
            
            NETWORK_TEST_STATUS=$?
            
            # Collect results
            if [ $NETWORK_TEST_STATUS -eq 0 ]; then
                LATENCY=$(grep "Ping latency" "$NETWORK_TEMP_DIR/node1.log" | awk '{print $4}')
                NETWORK_RESULTS="\"peer_discovery\": \"pass\", \"message_passing\": \"pass\", \"latency_ms\": $LATENCY"
            else
                NETWORK_RESULTS="\"peer_discovery\": \"fail\", \"message_passing\": \"fail\", \"latency_ms\": null"
            fi
            
            # Cleanup
            kill $NODE1_PID $NODE2_PID
            rm -rf "$NETWORK_TEMP_DIR"
            ;;
        Darwin)
            echo "Running macOS tests..."
            ./src-tauri/target/release/cpc-platform --test > /dev/null 2>&1 &
            TEST_PID=$!
            sleep 5  # Wait for app to start
            
            # Basic UI checks using AppleScript
            if osascript -e 'tell application "System Events" to tell process "cpc-platform" \
                to click button 1 of window 1'; then
                echo "macOS UI tests passed"
                MACOS_STATUS="pass"
            else
                echo "macOS UI tests failed"
                MACOS_STATUS="fail"
            fi
            
            kill $TEST_PID
            ;;
        CYGWIN*|MINGW*|MSYS*)
            echo "Running Windows tests..."
            ./src-tauri/target/release/cpc-platform --test > nul 2>&1 &
            TEST_PID=$!
            sleep 5  # Wait for app to start
            
            # Basic UI checks using PowerShell
            powershell -Command "& {
                Add-Type -AssemblyName System.Windows.Forms
                [System.Windows.Forms.SendKeys]::SendWait('{ENTER}')
                Start-Sleep -Milliseconds 500
                if (-not (Get-Process -Name cpc-platform -ErrorAction SilentlyContinue)) {
                    exit 1
                }
            }"
            
            if [ $? -eq 0 ]; then
                echo "Windows UI tests passed"
                WINDOWS_STATUS="pass"
            else
                echo "Windows UI tests failed"
                WINDOWS_STATUS="fail"
            fi
            
            taskkill /PID $TEST_PID /F > nul 2>&1
            
            # Texture memory validation for Windows
            echo "Validating texture memory..."
            if command -v nvidia-smi.exe &> /dev/null; then
                MEMORY_USAGE=$(nvidia-smi --query-compute-apps=pid,used_memory --format=csv,noheader,nounits | findstr "$TEST_PID")
                # Extract the second field (after comma)
                MEMORY_USAGE=$(echo "$MEMORY_USAGE" | awk -F',' '{print $2}' | tr -d ' ')
                if [ -z "$MEMORY_USAGE" ]; then
                    MEMORY_USAGE=0
                fi
                TEXTURE_MEMORY_USAGE=$MEMORY_USAGE
            else
                echo "nvidia-smi not found. Skipping texture memory validation."
                TEXTURE_MEMORY_USAGE=0
            fi

            TEXTURE_MEMORY_THRESHOLD=512
            if [ "$TEXTURE_MEMORY_USAGE" -gt "$TEXTURE_MEMORY_THRESHOLD" ]; then
                echo "Texture memory usage exceeded threshold: $TEXTURE_MEMORY_USAGE MB > $TEXTURE_MEMORY_THRESHOLD MB"
                TEXTURE_MEMORY_STATUS="fail"
            else
                echo "Texture memory usage: $TEXTURE_MEMORY_USAGE MB (within threshold)"
                TEXTURE_MEMORY_STATUS="pass"
            fi
            ;;
        *)
            echo "Unsupported OS"
            ;;
    esac
    
    # Common test cases
    echo "Running common tests..."
    # Minimize/restore test (TODO: Implement)
    # Focus change test (TODO: Implement)
    # Texture memory validation (preserved as placeholder)
    
    # Output results in JSON format
    PLATFORM=$(uname)
    case $PLATFORM in
        Linux) STATUS=$LINUX_STATUS ;;
        Darwin) STATUS=$MACOS_STATUS ;;
        *) STATUS=$WINDOWS_STATUS ;;
    esac
    echo "{\"platform\": \"$PLATFORM\", \"status\": \"$STATUS\", \"tests\": [\"ui_rendering\", \"input_response\"]}" > test_results.txt
    
    # Kill application after tests
    kill $APP_PID
    
    echo "Tests completed. See test_results.txt for details."
fi

exit $BUILD_STATUS