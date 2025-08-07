# Shtairir Standard Library Blocks Usage Mapping

This document maps the Shtairir demo applications to the standard library blocks they use,
providing a comprehensive overview of how the building blocks are composed into real-world workflows.

## Overview

The three demo applications showcase different combinations of Shtairir standard library blocks
to solve practical problems. This mapping helps understand which blocks are most commonly used
and how they interact with each other.

## Block Usage by Example

### Real-time Data Processing Pipeline

This example demonstrates stream processing and analytics workflows.

**Blocks Used:**

1. **collection.map** (`shared_packages/shtairir_blocks/specs/collection.map.toml`)
   - **Usage**: Transform raw sensor readings by adding processed timestamps
   - **Function**: `transform_function` - Adds metadata to sensor data
   - **Path**: `graphs/sensor_data_pipeline.toml` (nodes.map_transform)

2. **collection.filter** (`shared_packages/shtairir_blocks/specs/collection.filter.toml`)
   - **Usage**: Remove outlier sensor readings based on validity criteria
   - **Function**: `filter_predicate` - Validates temperature and humidity ranges
   - **Path**: `graphs/sensor_data_pipeline.toml` (nodes.filter_outliers)

3. **math.mean** (`shared_packages/shtairir_blocks/specs/math.mean.toml`)
   - **Usage**: Compute average temperature from processed readings
   - **Function**: Built-in statistical function
   - **Path**: `graphs/sensor_data_pipeline.toml` (nodes.compute_stats)

4. **string.format** (`shared_packages/shtairir_blocks/specs/string.format.toml`)
   - **Usage**: Generate human-readable summary reports
   - **Function**: Template-based string interpolation
   - **Path**: `graphs/sensor_data_pipeline.toml` (nodes.format_report)

5. **collection.reduce** (`shared_packages/shtairir_blocks/specs/collection.reduce.toml`)
   - **Usage**: Count the number of processed readings
   - **Function**: `count_function` - Simple increment accumulator
   - **Path**: `graphs/sensor_data_pipeline.toml` (nodes.count_readings)

6. **Custom Block**: `mock_sensor_data`
   - **Usage**: Generate realistic test data for the pipeline
   - **Function**: Random sensor data generation with realistic ranges
   - **Path**: `blocks/mock_sensor_data.toml` (nodes.mock_data)

**Workflow Pattern**: Data Ingestion → Transformation → Filtering → Analysis → Reporting

### User Profile Management Workflow

This example demonstrates data validation and transformation workflows.

**Blocks Used:**

1. **collection.map** (`shared_packages/shtairir_blocks/specs/collection.map.toml`)
   - **Usage**: Normalize user names and trim email addresses
   - **Functions**: 
     - `normalize_name_function` - Proper capitalization of names
     - `trim_email_function` - Remove whitespace from emails
     - `display_name_function` - Create user-friendly display names
   - **Path**: `graphs/user_profile_workflow.toml` (nodes.normalize_names, nodes.trim_emails, nodes.create_display_names)

2. **collection.filter** (`shared_packages/shtairir_blocks/specs/collection.filter.toml`)
   - **Usage**: Validate user profiles based on business rules
   - **Function**: `validation_predicate` - Check name, email, and age requirements
   - **Path**: `graphs/user_profile_workflow.toml` (nodes.validate_profiles)

3. **string.format** (`shared_packages/shtairir_blocks/specs/string.format.toml`)
   - **Usage**: Generate summary report of processing results
   - **Function**: Template-based reporting with counts
   - **Path**: `graphs/user_profile_workflow.toml` (nodes.format_summary)

4. **collection.reduce** (`shared_packages/shtairir_blocks/specs/collection.reduce.toml`)
   - **Usage**: Count total and valid profiles for reporting
   - **Function**: `count_function` - Simple counting accumulator
   - **Path**: `graphs/user_profile_workflow.toml` (nodes.count_total, nodes.count_valid)

5. **Custom Block**: `mock_user_data`
   - **Usage**: Generate realistic test user profiles
   - **Function**: Random profile generation with valid data patterns
   - **Path**: `blocks/mock_user_data.toml` (nodes.mock_data)

**Workflow Pattern**: Data Generation → Normalization → Validation → Enrichment → Reporting

### Machine Learning Feature Pipeline

This example demonstrates statistical processing and feature engineering workflows.

**Blocks Used:**

1. **collection.map** (`shared_packages/shtairir_blocks/specs/collection.map.toml`)
   - **Usage**: Feature engineering, statistical computation, and data normalization
   - **Functions**:
     - `feature_engineering_function` - Add polynomial and interaction features
     - `mean_function` - Compute feature means
     - `std_function` - Compute feature standard deviations
     - `normalize_function` - Apply z-score normalization
   - **Path**: `graphs/ml_feature_pipeline.toml` (nodes.add_features, nodes.compute_means, nodes.compute_stds, nodes.normalize_features)

2. **collection.random_sample** (`shared_packages/shtairir_blocks/specs/collection.random_sample.toml`)
   - **Usage**: Create training/test data splits
   - **Function**: Random sampling for dataset partitioning
   - **Path**: `graphs/ml_feature_pipeline.toml` (nodes.split_data)

3. **collection.stats_summary** (`shared_packages/shtairir_blocks/specs/collection.stats_summary.toml`)
   - **Usage**: Compute data quality metrics
   - **Function**: Built-in statistical summary computation
   - **Path**: `graphs/ml_feature_pipeline.toml` (nodes.quality_check)

4. **string.format** (`shared_packages/shtairir_blocks/specs/string.format.toml`)
   - **Usage**: Generate data quality reports
   - **Function**: Template-based reporting with statistical values
   - **Path**: `graphs/ml_feature_pipeline.toml` (nodes.format_report)

5. **Custom Block**: `mock_ml_data`
   - **Usage**: Generate realistic test ML datasets
   - **Function**: Random feature matrix generation with normal distributions
   - **Path**: `blocks/mock_ml_data.toml` (nodes.mock_data)

**Workflow Pattern**: Data Generation → Feature Engineering → Statistical Analysis → Normalization → Quality Assessment

## Block Usage Frequency

### Most Used Blocks

1. **collection.map** - Used in all 3 examples (3 times each)
   - Total usage: 9 times across all examples
   - Purpose: Data transformation and feature engineering

2. **collection.reduce** - Used in all 3 examples (1-2 times each)
   - Total usage: 5 times across all examples
   - Purpose: Aggregation and counting operations

3. **string.format** - Used in all 3 examples (1 time each)
   - Total usage: 3 times across all examples
   - Purpose: Report generation and output formatting

4. **collection.filter** - Used in 2 examples (1 time each)
   - Total usage: 2 times across all examples
   - Purpose: Data validation and outlier removal

### Specialized Blocks

1. **math.mean** - Used in 2 examples
   - Data Processing: Statistical analysis of sensor data
   - ML Features: Feature normalization

2. **collection.stats_summary** - Used in 1 example
   - ML Features: Data quality assessment

3. **collection.random_sample** - Used in 1 example
   - ML Features: Training/test data splitting

## Cross-Example Patterns

### Data Flow Patterns

1. **Generate → Transform → Validate → Report**
   - User Profiles example
   - Common in data processing applications

2. **Generate → Process → Analyze → Normalize → Assess**
   - ML Features example
   - Common in machine learning preprocessing

3. **Generate → Transform → Filter → Analyze → Report**
   - Data Processing example
   - Common in stream processing applications

### Function Composition Patterns

1. **Simple Mapping Functions**
   ```javascript
   fn transform(item) -> output {
     // Direct transformation
     return process(item)
   }
   ```

2. **Validation Predicates**
   ```javascript
   fn validate(item) -> bool {
     // Boolean conditions
     return condition1 && condition2
   }
   ```

3. **Accumulator Functions**
   ```javascript
   fn accumulate(acc, item) -> acc {
     // Aggregation logic
     return acc + value(item)
   }
   ```

4. **Feature Engineering Functions**
   ```javascript
   fn engineer(input) -> extended {
     // Complex transformation
     let result = input.clone()
     result.push(computed_feature(input))
     return result
   }
   ```

## Integration with Standard Library

### Direct Usage
All examples directly use standard library blocks through:
- TOML graph specifications referencing `stdlib.shtairir/` blocks
- Registry loading of standard library modules
- Runtime execution of standard block implementations

### Extension Patterns
Examples extend the standard library through:
- Custom block implementations for domain-specific operations
- Function definitions for complex business logic
- Graph compositions that create higher-level workflows

## Educational Insights

### Block Selection Guidelines

1. **For Data Transformation**: Use `collection.map` with custom functions
2. **For Data Validation**: Use `collection.filter` with predicate functions
3. **For Aggregation**: Use `collection.reduce` with accumulator functions
4. **For Statistics**: Use specialized math and collection blocks
5. **For Reporting**: Use `string.format` with template strings

### Performance Considerations

1. **Batch Processing**: All examples process collections in batch
2. **Function Complexity**: Simple functions perform better in map/filter operations
3. **Memory Usage**: Reduce operations can be memory-efficient with proper accumulators
4. **Parallelization**: Map operations are naturally parallelizable

### Error Handling Patterns

1. **Validation First**: Filter invalid data before processing
2. **Graceful Degradation**: Continue processing when possible
3. **Clear Error Messages**: Use meaningful error reporting
4. **Input Sanitization**: Normalize data before validation

## Future Block Usage

### Anticipated Needs

1. **Advanced Math Operations**
   - `math.stddev` for more detailed statistics
   - `math.correlation` for feature relationships

2. **Advanced Collection Operations**
   - `collection.group_by` for data categorization
   - `collection.sort` for ordered processing

3. **String Operations**
   - `string.regex_match` for complex pattern matching
   - `string.parse` for type conversion

4. **Date/Time Operations**
   - `datetime.parse` for temporal data processing
   - `datetime.format` for time-based reporting

## Conclusion

The Shtairir demo applications demonstrate that a relatively small set of standard library blocks
can be composed into powerful workflows for diverse applications. The core functional programming
blocks (`map`, `filter`, `reduce`) form the foundation, while specialized blocks provide
domain-specific capabilities.

The examples show that:
1. **80% of workflow needs** can be met with core collection and string operations
2. **Specialized domains** require targeted blocks (math, statistics, random sampling)
3. **Custom extensions** are essential for domain-specific requirements
4. **Composition patterns** are consistent across different application types

This mapping serves as a guide for:
- Understanding block usage patterns
- Planning standard library enhancements
- Designing new workflows
- Optimizing performance
- Teaching Shtairir concepts