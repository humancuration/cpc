import { invoke } from '@tauri-apps/api/tauri';

/**
 * Dispatches a command to the backend and handles the response
 * @param {string} commandName - The name of the command to execute
 * @param {Object} args - Arguments to pass to the command
 * @returns {Promise<Object>} The parsed JSON result from the command
 */
export async function dispatchCommand(commandName, args) {
    try {
        const result = await invoke('dispatch_command', {
            command: commandName,
            args: JSON.stringify(args)
        });
        return JSON.parse(result);
    } catch (error) {
        console.error(`Command ${commandName} failed:`, error);
        throw error;
    }
}