<script lang="ts">
  import type { GHUBApp, Command } from '$lib/types';

  interface CommandsTabProps {
    game: GHUBApp;
  }

  const { game }: CommandsTabProps = $props();

  let editingCommand = $state<{ categoryIndex: number; commandIndex: number } | null>(null);
  let newCommand = $state<Command>({ category: '', keystroke: [], name: '' });
  let keystrokeInput = $state('');
  
  // Group commands by category
  let groupedCommands = $state<Record<string, Command[]>>({});
  
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
    }
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

<div class="space-y-6">
  <!-- Current Commands -->
  <div>
    <h3 class="text-lg font-medium text-white mb-4">Current Commands</h3>
    
    {#if game.commands.length === 0}
      <p class="text-gray-400 italic">No commands configured</p>
    {:else}
      <div class="space-y-6">
        {#each Object.entries(groupedCommands) as [category, commands]}
          <div class="space-y-3">
            <h4 class="text-md font-semibold text-blue-300 border-b border-gray-600 pb-1">
              {category}
            </h4>
            <div class="space-y-1">
              {#each commands as command}
                <div class="group bg-gray-700 rounded px-3 py-2 flex items-center hover:bg-gray-600 transition-colors">
                  <div class="flex-1 flex items-center space-x-4">
                    <!-- Keys Input -->
                    <div class="flex items-center space-x-2 w-1/3">
                      <span class="text-xs text-gray-400 font-medium">Keys:</span>
                      <input
                        type="text"
                        value={command.keystroke.join(' + ')}
                        onblur={(e) => handleInputBlur(e, command, 'keystroke')}
                        placeholder="e.g., Ctrl + S"
                        class="bg-transparent text-gray-300 text-xs flex-1 focus:outline-none focus:bg-gray-600 focus:px-1 focus:py-0.5 focus:rounded"
                      />
                    </div>
                    <!-- Command Name Input -->
                    <div class="flex items-center space-x-2 flex-1">
                      <span class="text-xs text-gray-400 font-medium">Command:</span>
                      <input
                        type="text"
                        value={command.name}
                        onblur={(e) => handleInputBlur(e, command, 'name')}
                        class="bg-transparent text-white font-medium text-xs flex-1 focus:outline-none focus:bg-gray-600 focus:px-1 focus:py-0.5 focus:rounded"
                      />
                    </div>
                  </div>
                  
                  <!-- Delete Button -->
                  <button
                    onclick={() => handleDeleteCommand(command)}
                    class="opacity-0 group-hover:opacity-100 transition-opacity ml-2 p-1 text-red-400 hover:text-red-300 hover:bg-red-900/20 rounded"
                    title="Delete command"
                    aria-label="Delete command"
                  >
                    <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                    </svg>
                  </button>
                </div>
              {/each}
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>  <!-- Add Command Form -->
  <div class="bg-gray-700 rounded-lg p-3">
    <h3 class="text-md font-medium text-white mb-3">Add New Command</h3>
    
    <div class="space-y-3">
      <div>
        <label for="command-name" class="block text-sm font-medium text-gray-300 mb-2">
          Command Name
        </label>        <input
          id="command-name"
          type="text"
          bind:value={newCommand.name}
          placeholder="e.g., Quick Save"
          class="w-full px-2 py-1.5 bg-gray-600 border border-gray-500 rounded text-sm text-white placeholder-gray-400 focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-transparent"
        />
      </div>

      <div>
        <label for="command-category" class="block text-sm font-medium text-gray-300 mb-2">
          Category
        </label>
        <input
          id="command-category"
          type="text"
          bind:value={newCommand.category}
          placeholder="e.g., Game Controls"
          class="w-full px-2 py-1.5 bg-gray-600 border border-gray-500 rounded text-sm text-white placeholder-gray-400 focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-transparent"
        />
      </div>

      <div>
        <label for="command-keystroke" class="block text-sm font-medium text-gray-300 mb-2">
          Keystroke (separate multiple keys with +)
        </label>
        <input
          id="command-keystroke"
          type="text"
          value={keystrokeInput}
          oninput={handleKeystrokeInputChange}
          placeholder="e.g., Ctrl + S or F5"
          class="w-full px-2 py-1.5 bg-gray-600 border border-gray-500 rounded text-sm text-white placeholder-gray-400 focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-transparent"
        />
        <p class="text-xs text-gray-400 mt-1">
          Preview: {newCommand.keystroke.join(' + ') || 'None'}
        </p>
      </div>

      <button
        onclick={handleAddCommand}
        disabled={!newCommand.name.trim() || !newCommand.category.trim()}
        class="px-3 py-1.5 bg-green-600 hover:bg-green-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white text-sm rounded transition-colors"
      >
        Add Command
      </button>
    </div>
  </div>
</div>
