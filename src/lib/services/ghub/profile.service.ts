import { ws } from '../websocket';

/**
 * Service for managing G HUB profiles.
 * Provides high-level operations for profile management.
 */
class ProfileService {
	/**
	 * Delete a profile by application ID
	 * @param applicationId - The ID of the application/profile to delete
	 * @throws Error if the operation fails or times out
	 */
	async deleteProfile(applicationId: string): Promise<void> {
		console.log('[ProfileService] Deleting profile:', applicationId);

		try {
			await ws.request('REMOVE', '/profile', { id: applicationId });
			console.log('[ProfileService] Profile deleted successfully:', applicationId);
		} catch (error) {
			console.error('[ProfileService] Failed to delete profile:', error);
			throw new Error(
				`Failed to delete profile: ${error instanceof Error ? error.message : 'Unknown error'}`
			);
		}
	}

	/**
	 * Future: Add more profile operations here
	 * - updateProfile(id, data)
	 * - duplicateProfile(id)
	 * - exportProfile(id)
	 * - importProfile(data)
	 */
}

export const profileService = new ProfileService();
