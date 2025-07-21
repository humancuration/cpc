import { writable } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';

// Create writable stores for assets and lock events
export const currentPath = writable('');
export const assets = writable([]);
export const selectedAsset = writable(null);

// Function to handle asset-locked event
listen('asset-locked', (event) => {
    const { asset_id, user_id, user_name } = event.payload;
    assets.update(currentAssets =>
        currentAssets.map(asset =>
            asset.id === asset_id
                ? {...asset, lock_info: { user_id, user_name }}
                : asset
        )
    );
});

// Function to handle asset-unlocked event
listen('asset-unlocked', (event) => {
    const { asset_id } = event.payload;
    assets.update(currentAssets =>
        currentAssets.map(asset =>
            asset.id === asset_id
                ? {...asset, lock_info: null}
                : asset
        )
    );
});