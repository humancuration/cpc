# Health Module Migration Guide

This guide provides instructions for migrating from older health implementations to the new health module structure within `apps/health/`.

## Migration from Standalone Health Applications

If you were previously using standalone health applications in the `apps/` directory, follow these steps:

1. **Backup your data**: Before starting the migration, ensure you have backups of all health-related data.

2. **Update dependencies**: Remove any direct dependencies on the old health application and add the new `cpc-health` crate as a dependency.

3. **Update import paths**: Change all imports from the old health application to the new module:
   ```rust
   // Old import
   use health_tracker::domain::vital_signs::VitalSign;
   
   // New import
   use cpc_health::domain::vital_signs::VitalSign;
   ```

4. **Database migration**: Run the provided SQL migrations to update your database schema to match the new health module structure.

5. **Update service initialization**: If you were manually initializing health services, update your initialization code to use the new module structure.

6. **Update API endpoints**: The new health module provides standardized GraphQL APIs. Update any direct API calls to use the new GraphQL schema.

7. **Update UI components**: If you were using custom UI components, update them to use the new Bevy and Yew components provided by the health module.

## Database Migration

The health module uses the following database tables:

- `vital_signs`: Stores vital sign measurements
- `health_conditions`: Stores diagnosed health conditions
- `health_data_sharing_preferences`: Stores user preferences for health data sharing
- `health_alerts`: Stores health-related alerts

Run the SQL migrations in the main migrations directory to set up these tables.

## Configuration Changes

The health module uses the standard CPC configuration system. Any health-specific configuration should be moved to the appropriate configuration files.

## Breaking Changes

- Removed direct REST API endpoints in favor of GraphQL
- Changed data models to align with HIPAA compliance requirements
- Updated privacy settings to use granular consent management
- Replaced direct database access with repository pattern

## Support

If you encounter issues during migration, please consult the documentation or reach out to the development team for assistance.