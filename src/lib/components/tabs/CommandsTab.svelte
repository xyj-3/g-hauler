<script lang="ts">
  import type { GHUBApp, Command } from '$lib/types';

  interface CommandsTabProps {
    game: GHUBApp;
  }

  const { game }: CommandsTabProps = $props();

  let editingCommand = $state<{ categoryIndex: number; commandIndex: number } | null>(null);
  let newCommand = $state<Command>({ category: '', keystroke: [], name: '' });
  let keystrokeInput = $state('');
  let showCategoryDropdown = $state(false);
  
  // Group commands by category
  let groupedCommands = $state<Record<string, Command[]>>({});
  let existingCategories = $derived(Object.keys(groupedCommands));
  
  $effect(() => {
    const groups: Record<string, Command[]> = {};
    game.commands.forEach(command => {
      if (!groups[command.category]) {
        groups[command.category] = [];
      }
      groups[command.category].push(command);
    });
    groupedCommands = groups;
  });

  const handleAddCommand = () => {
    if (newCommand.name.trim() && newCommand.category.trim()) {
      game.commands = [...game.commands, { ...newCommand }];
      newCommand = { category: '', keystroke: [], name: '' };
      keystrokeInput = '';
      showCategoryDropdown = false;
    }
  };

  const selectCategory = (category: string) => {
    newCommand.category = category;
    showCategoryDropdown = false;
  };

  const handleDeleteCommand = (commandToDelete: Command) => {
    game.commands = game.commands.filter(command => command !== commandToDelete);
  };
  const parseKeystroke = (value: string) => value
    .split('+')
    .map(key => key.trim())
    .filter(key => key.length > 0);

  const handleKeystrokeInputChange = (event: Event) => {
    const target = event.target as HTMLInputElement;
    keystrokeInput = target.value;
    newCommand.keystroke = parseKeystroke(target.value);
  };

  const handleInputBlur = (event: Event, command: Command, field: 'name' | 'keystroke') => {
    const target = event.target as HTMLInputElement;
    if (!target) return;
    
    const commandIndex = game.commands.indexOf(command);
    if (commandIndex === -1) return;
    
    if (field === 'keystroke') {
      game.commands[commandIndex].keystroke = parseKeystroke(target.value);
    } else {
      game.commands[commandIndex][field] = target.value;
    }
  };
</script>

<div class="flex gap-6 h-full">
  <!-- Current Commands - Left Side (Scrollable) -->
  <div class="flex-1 flex flex-col min-w-0">
    <h3 class="text-lg font-semibold text-white mb-4">Current Commands</h3>

    <div class="flex-1 overflow-y-auto pr-2 min-h-0">
      {#if game.commands.length === 0}
        <div class="flex flex-col items-center justify-center h-full text-center py-12">
          <svg class="w-16 h-16 text-gray-600 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
          </svg>
          <p class="text-gray-400 text-sm">No commands configured yet</p>
          <p class="text-gray-500 text-xs mt-1">Add your first command using the form on the right</p>
        </div>
      {:else}
        <div class="space-y-6 pb-4">
          {#each Object.entries(groupedCommands) as [category, commands]}
            <div class="space-y-2">
              <div class="sticky top-0 bg-gray-800 z-10 flex items-center gap-2 px-3 py-2 mb-2 rounded border-l-2 border-gray-600">
                <svg class="w-3.5 h-3.5 text-gray-500 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z"></path>
                </svg>
                <span class="text-xs font-medium text-gray-400 uppercase tracking-wider">Category:</span>
                <h4 class="text-sm font-semibold text-gray-300">
                  {category}
                </h4>
              </div>
              <div class="space-y-1.5">
                {#each commands as command (command)}
                  {@const keystrokeDisplay = command.keystroke?.length ? command.keystroke.join(' + ') : ''}
                  <div class="group bg-gray-700/50 rounded-lg px-3 py-2 hover:bg-gray-700 transition-all hover:shadow-md border border-gray-700 hover:border-gray-600">
                    <div class="flex items-center gap-3">
                      <!-- Keystroke -->
                      <div class="flex items-center gap-2 w-1/3 min-w-0">
                        <input
                          type="text"
                          value={keystrokeDisplay}
                          onblur={(e) => handleInputBlur(e, command, 'keystroke')}
                          placeholder="e.g., Ctrl + S"
                          class="bg-transparent text-gray-300 text-sm flex-1 min-w-0 focus:outline-none focus:bg-gray-600 focus:px-2 focus:py-1 focus:-mx-2 focus:-my-1 focus:rounded transition-all"
                        />
                      </div>

                      <!-- Command Name -->
                      <div class="flex items-center gap-2 flex-1 min-w-0">
                        <span class="text-xs text-gray-500 flex-shrink-0">Command:</span>
                        <input
                          type="text"
                          value={command.name || ''}
                          onblur={(e) => handleInputBlur(e, command, 'name')}
                          placeholder="Command name"
                          class="bg-transparent text-white font-medium text-sm flex-1 min-w-0 focus:outline-none focus:bg-gray-600 focus:px-2 focus:py-1 focus:-mx-2 focus:-my-1 focus:rounded transition-all"
                        />
                      </div>

                      <!-- Delete Button -->
                      <button
                        onclick={() => handleDeleteCommand(command)}
                        class="opacity-0 group-hover:opacity-100 transition-opacity flex-shrink-0 p-1.5 text-red-400 hover:text-red-300 hover:bg-red-900/30 rounded"
                        title="Delete command"
                        aria-label="Delete command"
                      >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
                        </svg>
                      </button>
                    </div>
                  </div>
                {/each}
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  </div>

  <!-- Add Command Form - Right Side -->
  <div class="w-64 flex-shrink-0">
    <div class="bg-gray-750 rounded-lg p-3.5 shadow-lg border border-gray-600/50 sticky top-0">
      <div class="flex items-center gap-2 mb-3.5 pb-2.5 border-b border-gray-600/50">
        <div class="p-1 bg-blue-500/10 rounded">
          <svg class="w-4 h-4 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
          </svg>
        </div>
        <h3 class="text-sm font-semibold text-gray-200">Add Command</h3>
      </div>

      <div class="space-y-3">
        <div>
          <label for="command-name" class="block text-xs font-medium text-gray-400 mb-1">
            Name
          </label>
          <input
            id="command-name"
            type="text"
            bind:value={newCommand.name}
            placeholder="e.g., Quick Save"
            class="w-full px-2.5 py-1.5 bg-gray-800/60 border border-gray-600/50 rounded text-sm text-white placeholder-gray-500 focus:outline-none focus:ring-1 focus:ring-blue-500/50 focus:border-blue-500/50 focus:bg-gray-800 transition-all"
          />
        </div>

        <div class="relative">
          <label for="command-category" class="block text-xs font-medium text-gray-400 mb-1">
            Category
          </label>
          <div class="relative">
            <input
              id="command-category"
              type="text"
              bind:value={newCommand.category}
              onfocus={() => showCategoryDropdown = true}
              onblur={() => setTimeout(() => showCategoryDropdown = false, 200)}
              onkeydown={(e) => { if (e.key === 'Enter' || e.key === 'Tab') showCategoryDropdown = false; }}
              placeholder="Select or type..."
              class="w-full px-2.5 py-1.5 bg-gray-800/60 border border-gray-600/50 rounded text-sm text-white placeholder-gray-500 focus:outline-none focus:ring-1 focus:ring-blue-500/50 focus:border-blue-500/50 focus:bg-gray-800 pr-8 transition-all"
            />
            <button
              type="button"
              onclick={() => showCategoryDropdown = !showCategoryDropdown}
              class="absolute inset-y-0 right-0 flex items-center px-2 text-gray-500 hover:text-gray-300 transition-colors"
              aria-label="Toggle category dropdown"
            >
              <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
              </svg>
            </button>
          </div>
          {#if showCategoryDropdown && existingCategories.length > 0}
            <div class="absolute z-20 w-full mt-1 bg-gray-800 border border-gray-600/50 rounded shadow-xl max-h-36 overflow-y-auto">
              {#each existingCategories as category}
                <button
                  type="button"
                  onclick={() => selectCategory(category)}
                  class="w-full text-left px-2.5 py-1.5 text-sm text-gray-300 hover:bg-gray-700 hover:text-white transition-colors first:rounded-t last:rounded-b"
                >
                  {category}
                </button>
              {/each}
            </div>
          {/if}
        </div>

        <div>
          <label for="command-keystroke" class="block text-xs font-medium text-gray-400 mb-1">
            Keystroke
          </label>
          <input
            id="command-keystroke"
            type="text"
            value={keystrokeInput}
            oninput={handleKeystrokeInputChange}
            placeholder="e.g., Ctrl + S"
            class="w-full px-2.5 py-1.5 bg-gray-800/60 border border-gray-600/50 rounded text-sm text-white placeholder-gray-500 focus:outline-none focus:ring-1 focus:ring-blue-500/50 focus:border-blue-500/50 focus:bg-gray-800 transition-all"
          />
          <div class="mt-1.5 px-2 py-1 bg-gray-800/80 rounded border border-gray-700/30">
            <p class="text-xs text-gray-500">
              {newCommand.keystroke.join(' + ') || 'No keys'}
            </p>
          </div>
        </div>

        <button
          onclick={handleAddCommand}
          disabled={!newCommand.name.trim() || !newCommand.category.trim()}
          class="w-full px-3 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-700 disabled:cursor-not-allowed text-white text-sm font-medium rounded transition-all disabled:text-gray-500 flex items-center justify-center gap-1.5"
        >
          <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
          </svg>
          Add
        </button>
      </div>
    </div>
  </div>
</div>
