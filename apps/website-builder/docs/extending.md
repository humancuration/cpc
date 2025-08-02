# Extending the Website Builder Module

This document explains how to extend the website builder module with new features.

## Adding New Site Types

To add a new site type:

1. Add a new variant to the `SiteType` enum in `src/domain/models.rs`
2. Add a new struct for the site type's data
3. Update the database schema to accommodate the new site type
4. Update the repository to handle the new site type
5. Update the GraphQL types to include the new site type
6. Update the services to handle the new site type
7. Add integration with external services if needed (e.g., gRPC clients)

## Adding New Template Types

To add a new template type:

1. Add a new variant to the `TemplateType` enum in `src/domain/models.rs`
2. Update the database schema if needed
3. Update the template service to handle the new template type
4. Update the GraphQL types to include the new template type

## Adding New Analytics Metrics

To add new analytics metrics:

1. Add new fields to the `AnalyticsReport` struct in `src/domain/models.rs`
2. Update the database schema to store the new metrics
3. Update the analytics service to calculate the new metrics
4. Update the GraphQL types to include the new metrics

## Adding New Media Types

To add support for new media types:

1. Add new variants to the `SectionType` and `SectionContent` enums in `src/domain/models.rs`
2. Update the media processor to handle the new media types
3. Update the GraphQL types to include the new media types
4. Update the frontend components to render the new media types

## Adding New Form Field Types

To add new form field types:

1. Add a new variant to the `FormFieldType` enum in `src/domain/models.rs`
2. Update the GraphQL types to include the new form field type
3. Update the frontend components to render the new form field type

## Adding New GraphQL Operations

To add new GraphQL operations:

1. Add new queries, mutations, or subscriptions to the GraphQL schema in `src/web/graphql.rs`
2. Implement the resolver functions
3. Update the root query, mutation, and subscription types in the main backend

## Adding New REST Endpoints

To add new REST endpoints:

1. Add new route handlers in `src/web/routes.rs`
2. Add the new routes to the router in `src/web/module.rs`
3. Implement the handler functions

## Adding New Database Tables

To add new database tables:

1. Create a new migration file in `apps/backend/migrations/`
2. Add the new table schema to the migration file
3. Update the domain models if needed
4. Update the repository to interact with the new tables
5. Update the services to use the new tables

## Adding New Value Objects

To add new value objects:

1. Add a new struct and error enum in `src/domain/value_objects.rs`
2. Implement validation logic
3. Add tests for the new value object
4. Use the new value object in the domain models

## Adding New Services

To add new services:

1. Create a new service file in `src/application/`
2. Define the service struct and implementation
3. Add the new service to `src/application/mod.rs`
4. Update the module initialization in `src/web/module.rs`
5. Add the new service to the GraphQL schema if needed

## Adding New Repositories

To add new repositories:

1. Add new methods to the existing repository or create a new repository in `src/infrastructure/`
2. Implement the database queries
3. Add tests for the new repository methods
4. Update the services to use the new repository methods

## Adding New Web Adapters

To add new web adapters:

1. Create a new adapter file in `src/web/`
2. Implement the adapter logic
3. Add the new adapter to `src/web/mod.rs`
4. Update the module initialization in `src/web/module.rs`

## Testing New Features

When adding new features, make sure to:

1. Write unit tests for new domain logic
2. Write integration tests for new repository methods
3. Write tests for new GraphQL operations
4. Write tests for new REST endpoints
5. Update existing tests if needed

## Documentation

When adding new features, make sure to:

1. Update the README.md file
2. Update the usage guide
3. Update the architecture documentation if needed
4. Update the GraphQL documentation
5. Update the database documentation if needed

## Adding Fundraising Campaign Integration

To add fundraising campaign integration:

1. Add new variants to the `SiteType` enum for fundraising campaigns
2. Add new structs for campaign data and campaign types
3. Create a gRPC client to communicate with the fundraising service
4. Update the database schema to store campaign information
5. Update the repository to handle campaign data
6. Update the services to create and manage campaigns
7. Add new GraphQL types and mutations for campaigns
8. Create frontend components for campaign creation and management