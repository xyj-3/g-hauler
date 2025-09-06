<script lang="ts">
	import { open as openDialog } from '@tauri-apps/plugin-dialog';

	interface AddGameOptions {
		name?: string;
		installed?: boolean;
	}

	let { name = '', installed = true }: AddGameOptions = $props();

	let gameName = $state(name);
	let gameInstalled = $state(installed);
	let filePath = $state('');

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

	function handleSubmit(event: SubmitEvent) {
		event.preventDefault();
		console.log({
			name: gameName,
			installed: gameInstalled,
			filePath: filePath
		});
	}
</script>

<h1>Add Games</h1>

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

	<div class="flex justify-center">
		<button
			type="submit"
			class="bg-blue-600 text-white text-sm py-2 px-6 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-colors"
		>
			Add Game
		</button>
	</div>
</form>