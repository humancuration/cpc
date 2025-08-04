# Manual File Moves Required

The following file moves need to be performed manually due to tool limitations. Please execute these moves at your earliest convenience:

## Messenger Shared Package
- Move `shared_packages/messenger/AUTH_IMPLEMENTATION.md` → `shared_packages/messenger/docs/AUTH_IMPLEMENTATION.md`

## Messenger Win64 App
- Move `apps/messenger_win64/docs/005-websocket-token-refresh.md` → `apps/messenger_win64/docs/adr/005-websocket-token-refresh.md`
- Move `apps/messenger_win64/docs/006-graphql-auth-middleware.md` → `apps/messenger_win64/docs/adr/006-graphql-auth-middleware.md`
- Move `apps/messenger_win64/docs/007-grpc-client-implementation.md` → `apps/messenger_win64/docs/adr/007-grpc-client-implementation.md`

## Post-Move Verification
After moving, verify that these files exist in their new locations and that their content matches the originals.