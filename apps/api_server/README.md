# API Server

The API server for the CPC platform provides GraphQL endpoints for all core features including social, volunteer, and skill exchange functionality.

## Features

- GraphQL API for volunteer tracking
- GraphQL API for skill exchange marketplace
- Integration with wallet, notification, and social systems
- Real-time subscriptions for volunteer and skill events

## GraphQL Schema

The API server exposes the following GraphQL schema:

### Volunteer Features

- `logVolunteerHours` - Log volunteer hours for a user
- `verifyVolunteerHours` - Verify volunteer hours (organization admin only)
- `convertToDabloons` - Convert verified volunteer hours to Dabloons
- `myVolunteerActivities` - Get all volunteer activities for the current user
- `myVerifiedVolunteerActivities` - Get verified volunteer activities for the current user

### Skill Exchange Features

- `createSkillListing` - Create a new skill listing
- `updateSkillListing` - Update an existing skill listing
- `deactivateSkillListing` - Deactivate a skill listing
- `claimSkillListing` - Claim a skill listing
- `acceptClaim` - Accept a claim on a skill listing
- `rejectClaim` - Reject a claim on a skill listing
- `completeSkillExchange` - Complete a skill exchange
- `searchSkillListings` - Search for active skill listings
- `mySkillListings` - Get skill listings by provider
- `mySkillClaims` - Get claims by claimant

## Testing

The API server includes comprehensive unit and integration tests:

```bash
cargo test
```

## License

This software is licensed under the CPC License.