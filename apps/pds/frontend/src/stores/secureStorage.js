import { invoke } from '@tauri-apps/api';

/**
 * Secure storage wrapper for sensitive settings and credentials
 */
export class SecureStorage {
  /**
   * Store sensitive data securely
   * @param {string} key - The key to store the data under
   * @param {string} value - The value to store
   * @returns {Promise<void>}
   */
  static async store(key, value) {
    try {
      await invoke('secure_store', { key, value });
    } catch (error) {
      console.error('Secure storage error:', error);
      throw new Error(`Failed to store secure data: ${error.message}`);
    }
  }

  /**
   * Retrieve sensitive data from secure storage
   * @param {string} key - The key to retrieve
   * @returns {Promise<string|null>} - The stored value or null if not found
   */
  static async retrieve(key) {
    try {
      return await invoke('secure_retrieve', { key });
    } catch (error) {
      console.error('Secure storage error:', error);
      return null;
    }
  }

  /**
   * Remove sensitive data from secure storage
   * @param {string} key - The key to remove
   * @returns {Promise<void>}
   */
  static async remove(key) {
    try {
      await invoke('secure_remove', { key });
    } catch (error) {
      console.error('Secure storage error:', error);
      throw new Error(`Failed to remove secure data: ${error.message}`);
    }
  }

  /**
   * Get the total size of secure storage
   * @returns {Promise<number>} - Size in bytes
   */
  static async size() {
    try {
      return await invoke('secure_storage_size');
    } catch (error) {
      console.error('Secure storage error:', error);
      return 0;
    }
  }
}

// Convenience functions for backward compatibility
export const secureStore = SecureStorage.store;
export const secureRetrieve = SecureStorage.retrieve;
export const secureRemove = SecureStorage.remove;
export const secureStorageSize = SecureStorage.size;