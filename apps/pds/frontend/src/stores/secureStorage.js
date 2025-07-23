import { invoke } from '@tauri-apps/api/tauri'

/**
 * Securely stores a key-value pair using desktop app's secure storage
 * @param {string} key - The key to store
 * @param {string} value - The value to store (will be encrypted)
 * @returns {Promise<void>}
 */
export async function secureStore(key, value) {
    return invoke('secure_store', { key, value });
}

/**
 * Retrieves a value from secure storage
 * @param {string} key - The key to retrieve
 * @returns {Promise<string|null>} The decrypted value or null if key doesn't exist
 */
export async function secureRetrieve(key) {
    return invoke('secure_retrieve', { key });
}

/**
 * Deletes a key-value pair from secure storage
 * @param {string} key - The key to delete
 * @returns {Promise<void>}
 */
export async function secureDelete(key) {
    return invoke('secure_delete', { key });
}

/**
 * Lists all keys stored in secure storage
 * @returns {Promise<string[]>} Array of key names
 */
export async function secureListKeys() {
    return invoke('secure_list_keys');
}

/**
 * Utility function to check if a key exists in secure storage
 * @param {string} key - The key to check
 * @returns {Promise<boolean>} True if the key exists
 */
export async function secureHasKey(key) {
    const value = await secureRetrieve(key);
    return value !== null;
}

/**
 * Clears all keys from secure storage
 * @returns {Promise<void>}
 */
export async function secureClear() {
    const keys = await secureListKeys();
    await Promise.all(keys.map(key => secureDelete(key)));
}