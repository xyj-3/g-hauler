<script lang="ts">
	import { open as openDialog } from '@tauri-apps/plugin-dialog';
	import { ws } from '$lib/services/websocket';

	interface AddGameOptions {
		name?: string;
		installed?: boolean;
	}

	let { name = '', installed = true }: AddGameOptions = $props();

	let gameName = $state(name);
	let gameInstalled = $state(installed);
	let filePath = $state('');
	let isSubmitting = $state(false);
	let submitMessage = $state('');

	async function handleBrowseFile() {
		const selected = await openDialog({
			directory: false,
			multiple: false,
			title: 'Select File'
		});
		if (typeof selected === 'string') {
			filePath = selected;
		}
	}

	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault();

		if (!gameName.trim() || !filePath.trim()) {
			submitMessage = 'Please fill in all required fields';
			return;
		}

		isSubmitting = true;
		submitMessage = '';

		try {
			// Check if WebSocket is connected
			if (!(await ws.isConnected())) {
				throw new Error('WebSocket not connected. Please connect first.');
			}

			// Send the WebSocket message
			await ws.setApplication(filePath, gameName, { isInstalled: gameInstalled });

			submitMessage = 'Game added successfully!';
			
			// Reset form
			gameName = '';
			filePath = '';
			gameInstalled = true;
			
		} catch (error) {
			console.error('Failed to add game:', error);
			submitMessage = error instanceof Error ? error.message : 'Failed to add game';
		} finally {
			isSubmitting = false;
		}
	}
</script>

<main class="w-full min-h-full px-6 py-4 text-white">
	<h1 class="text-xl font-dm-sans mb-1">Add Games</h1>

	<p class="text-xs text-gray-400 mb-3">
		Add game profiles to G HUB. Use Fix Game Detection to automatically detect installed games.
	</p>

	<!-- Note: For a truly reactive status indicator, you'd need a separate component that polls the connection status -->

	<form onsubmit={handleSubmit} class="max-w-md mx-auto mt-8 space-y-6">
	<div class="space-y-2">
		<label for="game-name" class="block text-sm font-medium text-white">
			Game Name
		</label>
		<input
			id="game-name"
			type="text"
			bind:value={gameName}
			placeholder="Enter game name"
			class="w-full px-3 py-2 text-sm border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
		/>
	</div>

	<div class="space-y-2">
		<label for="file-path" class="block text-sm font-medium text-white">
			File Path
		</label>
		<div class="flex gap-2">
			<input
				id="file-path"
				type="text"
				bind:value={filePath}
				placeholder="Enter file path or click browse"
				class="flex-1 px-3 py-2 text-sm border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
			/>
			<button
				type="button"
				onclick={handleBrowseFile}
				class="px-3 py-2 text-sm bg-gray-600 text-white rounded-md hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-500 transition-colors"
			>
				Browse
			</button>
		</div>
	</div>

	<div class="space-y-4">
		<div class="flex items-center">
			<input
				id="game-installed"
				type="checkbox"
				bind:checked={gameInstalled}
				class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
			/>
			<label for="game-installed" class="ml-2 block text-sm text-white">
				Installed
			</label>
		</div>
	</div>

	<!-- Submit Message -->
	{#if submitMessage}
		<div class="p-3 rounded-md {submitMessage.includes('success') ? 'bg-green-900 text-green-200' : 'bg-red-900 text-red-200'}">
			<p class="text-sm">{submitMessage}</p>
		</div>
	{/if}

	<div class="flex justify-center">
		<button
			type="submit"
			disabled={isSubmitting}
			class="bg-btn text-white text-sm py-2 px-6 rounded-md hover:bg-btn-hover focus:outline-none focus:ring-2 focus:ring-btn-disabled focus:ring-offset-2 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
		>
			{isSubmitting ? 'Adding Game...' : 'Add Game'}
		</button>
	</div>
	</form>
</main>