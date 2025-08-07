# Machine Learning Feature Pipeline

A pipeline for preparing data for machine learning models with feature engineering and normalization.

## Overview

This example demonstrates a machine learning feature engineering pipeline that:
1. Generates mock ML dataset with multiple features
2. Performs feature engineering (polynomial terms, interactions)
3. Computes statistical measures (mean, standard deviation)
4. Normalizes features using z-score normalization
5. Splits data into training and test sets
6. Performs data quality checks
7. Generates a summary report

## Components Used

- `math.vector_add` for feature engineering
- `math.mean` and `math.stddev` for normalization
- `collection.random_sample` for creating training/test splits
- `collection.stats_summary` for data quality checks
- `conversion.parse_json` for handling complex data structures

## Usage

To run the demo:

```bash
cargo run -p shtairir_demos_ml_features
```

## Graph Structure

The pipeline is defined as a TOML graph specification that composes standard library blocks:

```
Mock Data → Feature Engineering → Compute Stats → Normalize Features → Quality Check
    ↓            ↓                  ↓                ↓
 Sample Count  Feature Count   Mean/Std Dev      Format Report
```

## Feature Engineering

- Polynomial features (squared terms)
- Interaction features (products of consecutive features)
- Z-score normalization ((x - mean) / std)

## Data Quality Metrics

- Overall mean and standard deviation
- Value ranges
- Sample statistics

## Performance Metrics

The demo includes execution time measurement and data quality reporting.

## Customization

You can modify the `sample_count` and `feature_count` parameters in the main function to process different dataset sizes.