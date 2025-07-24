-- Drop user_preferences table and associated objects
DROP TRIGGER IF EXISTS update_user_preferences_updated_at_trigger ON user_preferences;
DROP FUNCTION IF EXISTS update_user_preferences_updated_at();
DROP INDEX IF EXISTS idx_user_preferences_user_id;
DROP TABLE IF EXISTS user_preferences;