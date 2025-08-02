# Website Builder GraphQL API

This document describes the GraphQL API provided by the website builder module.

## Schema Overview

The module provides the following GraphQL operations:

### Queries

- `site(id: ID!)` - Get site details
- `sitesByOwner(ownerId: ID!)` - List all sites for a cooperative member
- `templates` - List available templates
- `siteAnalytics(siteId: ID!, period: AnalyticsPeriod)` - Get analytics

### Mutations

- `createSite(input: CreateSiteInput!)` - Create a new site
- `updateSiteSettings(input: UpdateSiteSettingsInput!)` - Update site settings
- `updateSiteContent(input: UpdateSiteContentInput!)` - Update site content
- `publishSite(siteId: ID!)` - Publish a site
- `trackLinkClick(linkId: ID!)` - Track a link click
- `createFundraisingCampaign(input: CreateSiteInput!)` - Create a fundraising campaign site

### Subscriptions

- `sitePublished(siteId: ID!)` - Site publishing updates
- `linkClicked(siteId: ID!)` - Link click events

## Types

### Site

```graphql
type Site {
  id: ID!
  ownerId: ID!
  siteType: SiteType!
  name: String!
  customDomain: String
  primaryColor: String!
  secondaryColor: String!
  fontFamily: String!
  isPublished: Boolean!
  createdAt: DateTime!
  updatedAt: DateTime!
}
```

### SiteType

```graphql
type SiteType {
  fullWebsite: FullWebsiteData
  linkInBio: LinkInBioData
  fundraisingCampaign: FundraisingCampaignData
}
```

### FullWebsiteData

```graphql
type FullWebsiteData {
  templateId: ID!
  pages: [Page!]!
}
```

### LinkInBioData

```graphql
type LinkInBioData {
  profileImage: MediaAsset
  headline: String!
  description: String!
  links: [LinkItem!]!
  clickCount: Int!
}
```

### FundraisingCampaignData

```graphql
type FundraisingCampaignData {
  campaignId: ID!
  campaignTitle: String!
  campaignDescription: String!
  campaignType: CampaignType!
  goalAmount: Int
  currentAmount: Int!
  startDate: DateTime!
  endDate: DateTime
}
```

### Template

```graphql
type Template {
  id: ID!
  name: String!
  description: String
  templateType: TemplateType!
  previewImageCid: String!
  structure: TemplateStructure!
  isDefault: Boolean!
  createdAt: DateTime!
  updatedAt: DateTime!
}
```

### AnalyticsReport

```graphql
type AnalyticsReport {
  siteId: ID!
  periodStart: DateTime!
  periodEnd: DateTime!
  totalViews: Int!
  uniqueVisitors: Int!
  linkClicks: JSON!
  pageViews: JSON!
}
```

## Input Types

### CreateSiteInput

```graphql
input CreateSiteInput {
  name: String!
  siteType: SiteTypeInput!
}
```

### SiteTypeInput

```graphql
input SiteTypeInput {
  fullWebsite: FullWebsiteDataInput
  linkInBio: LinkInBioDataInput
  fundraisingCampaign: FundraisingCampaignDataInput
}
```

### FundraisingCampaignDataInput

```graphql
input FundraisingCampaignDataInput {
  campaignTitle: String!
  campaignDescription: String!
  campaignType: CampaignType!
  goalAmount: Int
  startDate: DateTime!
  endDate: DateTime
}
```

### UpdateSiteSettingsInput

```graphql
input UpdateSiteSettingsInput {
  siteId: ID!
  name: String
  customDomain: String
  primaryColor: String
  secondaryColor: String
  fontFamily: String
}
```

### CampaignType

```graphql
enum CampaignType {
  CooperativeMembership
  PureDonation
  RegCF
  RegA
  RegD
}
```

## Enums

### TemplateType

```graphql
enum TemplateType {
  FullWebsite
  LinkInBio
}
```

## Usage

To use the GraphQL API, you need to include the website builder module's query, mutation, and subscription types in your main GraphQL schema.

The module's GraphQL components can be imported and added to your schema like this:

```rust
use cpc_website_builder::web::graphql::{WebsiteBuilderQuery, WebsiteBuilderMutation, WebsiteBuilderSubscription};

// In your schema definition:
#[derive(MergedObject, Default)]
pub struct RootQuery(
    // ... other queries
    WebsiteBuilderQuery
);

#[derive(MergedObject, Default)]
pub struct RootMutation(
    // ... other mutations
    WebsiteBuilderMutation
);

#[derive(MergedSubscription, Default)]
pub struct RootSubscription(
    // ... other subscriptions
    WebsiteBuilderSubscription
);
```

## Authentication

All operations require authentication. The user ID is typically extracted from the authentication token and passed to the services.

## Error Handling

The API uses GraphQL's built-in error handling mechanism. Errors are returned in the `errors` field of the response.