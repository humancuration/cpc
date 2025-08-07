# User Profile Management Workflow

A workflow for managing user profile data with validation and transformation.

## Overview

This example demonstrates a user profile management workflow that:
1. Generates mock user profile data
2. Normalizes names (proper capitalization, trimming)
3. Trims whitespace from email addresses
4. Validates profile data (email format, age requirements)
5. Creates display names for valid profiles
6. Generates a summary report of processing results

## Components Used

- `string.trim` and `string.format` for data normalization
- `conversion.parse_json` for handling profile data
- `collection.filter` for data validation
- `math.fixed_multiply` for privacy-preserving data transformations
- `string.concat` for creating derived fields

## Usage

To run the demo:

```bash
cargo run -p shtairir_demos_user_profiles
```

## Graph Structure

The workflow is defined as a TOML graph specification that composes standard library blocks:

```
Mock Data → Normalize Names → Trim Emails → Validate → Create Display Names
    ↓           ↓              ↓
Count Total  Count Valid   Format Summary
```

## Validation Rules

- Names must not be empty
- Email must have valid format (contains @ and .)
- Age must be at least 13 (COPPA compliance)
- Display names are created from first name and user ID

## Performance Metrics

The demo includes execution time measurement to show workflow performance.

## Customization

You can modify the `profile_count` parameter in the main function to process different numbers of user profiles.