# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

G Hauler is a cross-platform desktop application built with Tauri v2 that manages Logitech G HUB data files and provides game detection patching functionality. The app is designed for Windows and macOS platforms.

## Technology Stack

- **Backend**: Rust with Tauri v2
- **Frontend**: Svelte 5 + SvelteKit 2 + TypeScript
- **Styling**: Tailwind CSS 4
- **Build Tool**: Vite 6
- **Package Manager**: pnpm

## Development Commands

### Setup
```bash
# Install dependencies
pnpm install
```

### Development
```bash
# Run in development mode (starts both frontend and backend)
pnpm tauri dev

# Run frontend only (for UI development)
pnpm dev

# Build the application
pnpm tauri build

# Type checking
pnpm check

# Type checking in watch mode
pnpm check:watch
```

### Tauri-specific Commands
```bash
# Access Tauri CLI directly
pnpm tauri --help

# Generate Tauri assets
pnpm tauri icon

# Show Tauri info
pnpm tauri info
```

## Architecture Overview

### Backend (Rust/Tauri)
The Tauri backend is structured with clear separation of concerns:

- **`lib.rs`**: Main entry point, plugin registration, and invoke handler setup
- **`models/`**: Data structures and type definitions
  - `ghub_app.rs`: Core G HUB application data structures (GHUBApp, Detection variants, Commands, etc.)
  - `app_state.rs`: Application state management
  - `applications_data.rs`: Application data handling
  - `path_validation.rs`: File system validation logic
- **`applications.rs`**: Game application management and operations
- **`store.rs`**: Persistent data storage using Tauri's store plugin
- **`ghub.rs`**: G HUB specific functionality
- **`validation.rs`**: Path and data validation utilities
- **`util.rs`**: General utility functions

### Frontend (Svelte/TypeScript)
The frontend follows SvelteKit conventions with Svelte 5 runes:

- **Routes**: Located in `src/routes/`
  - `+layout.svelte`: Main layout with splash screen, sidebar, and bottom bar
  - `+page.svelte`: Home page
  - `add_games/`, `detection/`, `settings/`: Feature-specific pages
- **Components**: Organized in `src/lib/components/`
  - `layout/`: Core layout components (TitleBar, Sidebar, BottomBar)
  - `modal/`: Modal dialogs for various interactions
  - `tabs/`: Tab components for different sections (General, Commands, Detection)
- **Types**: TypeScript interfaces in `src/lib/types.ts` that mirror Rust structs
- **Constants**: Shared constants in `src/lib/constants.ts`

### Key Data Models
The application revolves around the `GHUBApp` structure which represents a game configuration with:
- Application metadata (name, ID, version)
- Detection methods (Steam, Epic Games, Windows Registry, etc.)
- Commands and keybindings
- Visual customization (poster URL, category colors)

### IPC Communication
The frontend communicates with the Tauri backend via invoke calls:
- Store operations: `store_get_key`, `store_set_key`
- Application management: `get_applications`, `update_application`, `save_applications_to_disk`
- Validation: `validate_paths`
- Utilities: `get_pipeline_path`

## Important Configuration

### Tauri Configuration
- **Window**: Custom decorations disabled, transparent background
- **Bundle**: Icons configured for multiple platforms
- **Plugins**: fs, store, opener, dialog
- **Development URL**: http://localhost:1420

### Build Configuration
- **Frontend build**: Uses Vite with SvelteKit static adapter
- **Tauri bundling**: Configured for Windows and macOS targets
- **Development**: Auto-reload enabled with HMR

### Svelte 5 Usage
This project uses Svelte 5 with runes:
- Use `$state()` for reactive variables instead of `let`
- Use `$derived()` for computed values instead of `$:`
- Use `$effect()` for side effects instead of `$: {}`
- Use standard HTML event attributes (`onclick`) instead of Svelte directives (`on:click`)
- Import runes from 'svelte': `import { $state, $derived, $effect } from 'svelte'`

### Styling Approach
- Tailwind CSS 4 for all styling
- No CSS modules or styled-components
- Custom decorations disabled for native window appearance
- Responsive design considerations for desktop form factors

## Data Management

The application manages G HUB data files located at:
- **Windows**: `C:\ProgramData\LGHUB` (default)
- The data path is configurable and stored using Tauri's store plugin
- Key files: `applications.json`, `current.json`, `version.json`

## Development Guidelines

### Code Style
- Follow the existing Cursor rules in `.cursor/rules/g-hauler.mdc`
- Use TypeScript throughout the frontend
- Maintain type safety between Rust structs and TypeScript interfaces
- Use descriptive variable names and prefix event handlers with "handle"

### Component Structure
- Keep components small and focused on single responsibilities
- Use props for configuration and composition via slots
- Implement proper error handling and loading states
- Ensure accessibility with proper ARIA attributes and keyboard navigation

### State Management
- Prefer Svelte 5 `$state` rune over stores for local state
- Use Tauri's store plugin for persistent data
- Maintain application state in the Rust backend when possible

## Testing and Building

The project includes type checking via `svelte-check` and uses Tauri's built-in development and build processes. The application is configured to work as a static site (SSG) since Tauri doesn't support SSR.