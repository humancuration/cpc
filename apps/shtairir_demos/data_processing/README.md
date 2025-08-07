# Real-time Data Processing Pipeline

A stream processing workflow that ingests sensor data, processes it, and generates analytics.

## Overview

This example demonstrates a real-time data processing pipeline that:
1. Generates mock sensor data (temperature, humidity readings)
2. Transforms and normalizes the data
3. Filters out outlier readings
4. Extracts temperature values for statistical analysis
5. Computes mean temperature
6. Generates a human-readable report

## Components Used

- `collection.map` to transform raw sensor readings
- `collection.filter` to remove outliers
- `math.mean` for statistical analysis
- `string.format` to create human-readable reports
- `collection.reduce` as a stateful-breaker for counting readings

## Usage

To run the demo:

```bash
cargo run -p shtairir_demos_data_processing
```

## Graph Structure

The pipeline is defined as a TOML graph specification that composes standard library blocks:

```
Mock Data → Transform → Filter → Extract Temps → Compute Mean
                ↓                    ↓
            Count Readings      Format Report
```

## Performance Metrics

The demo includes performance metrics collection to measure:
- Processing latency
- Execution time
- Throughput statistics

## Customization

You can modify the `reading_count` parameter in the main function to process different amounts of data.