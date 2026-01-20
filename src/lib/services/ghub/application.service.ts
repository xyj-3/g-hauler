import { ws } from '../websocket';
import type { ApplicationPayload } from '$lib/types';

/**
 * Service for managing G HUB applications.
 * Provides high-level operations for application management.
 *
 * This service demonstrates the pattern for creating domain-specific
 * websocket operations. Add new methods here as needed.
 */
class ApplicationService {
	/**
	 * Request all applications from G HUB (fire-and-forget)
	 * The response will come via websocket events and be handled by stores
	 */
	async requestApplications(): Promise<void> {
		console.log('[ApplicationService] Requesting all applications');

		try {
			await ws.send('GET', '/applications');
			console.log('[ApplicationService] Application request sent');
		} catch (error) {
			console.error('[ApplicationService] Failed to request applications:', error);
			throw new Error(
				`Failed to request applications: ${error instanceof Error ? error.message : 'Unknown error'}`
			);
		}
	}

	/**
	 * Get a single application by ID (with response)
	 * @param applicationId - The ID of the application to fetch
	 * @returns The application data
	 */
	async getApplication(applicationId: string): Promise<ApplicationPayload> {
		console.log('[ApplicationService] Fetching application:', applicationId);

		try {
			const response = await ws.request<ApplicationPayload>(
				'GET',
				'/application',
				{ id: applicationId }
			);
			console.log('[ApplicationService] Application fetched successfully');
			return response;
		} catch (error) {
			console.error('[ApplicationService] Failed to fetch application:', error);
			throw new Error(
				`Failed to fetch application: ${error instanceof Error ? error.message : 'Unknown error'}`
			);
		}
	}

	/**
	 * Update an application
	 * @param application - The application data to update
	 */
	async updateApplication(application: Partial<ApplicationPayload>): Promise<void> {
		console.log('[ApplicationService] Updating application:', application.applicationId);

		try {
			await ws.request('SET', '/application', application);
			console.log('[ApplicationService] Application updated successfully');
		} catch (error) {
			console.error('[ApplicationService] Failed to update application:', error);
			throw new Error(
				`Failed to update application: ${error instanceof Error ? error.message : 'Unknown error'}`
			);
		}
	}

	/**
	 * Future: Add more application operations here
	 * - createApplication(data)
	 * - listApplications()
	 * - searchApplications(query)
	 * - getApplicationCommands(id)
	 * - setApplicationCommands(id, commands)
	 */
}

export const applicationService = new ApplicationService();
