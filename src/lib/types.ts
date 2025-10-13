// TypeScript interfaces matching the Rust structs

export interface GHUBApp {
  applicationId: string;
  categoryColors: CategoryColor[];
  commands: Command[];
  detection: Detection[];
  name: string;
  posterTitlePosition: string;
  posterUrl: string;
  version: number;
}

export interface CategoryColor {
  hex: string;
  tag: string;
}

export interface Command {
  category: string;
  keystroke: string[];
  name: string;
}

export type Detection = 
  | { steam: SteamApp }
  | { winRegistry: WinRegistry }
  | { epicGames: EpicGames }
  | { osxBundle: OsxBundle }
  | { uplay: Uplay }
  | { gogGalaxy: GogGalaxy }
  | { humbleApp: HumbleApp }
  | { riotGames: RiotGames }
  | { glob: string }
  | any;

export interface SteamApp {
  appId: string;
}

export interface WinRegistry {
  executable: string;
  registryKey: string;
  registryPath: string;
}

export interface EpicGames {
  appName: string;
}

export interface OsxBundle {
  bundleId: string;
  bundlePath: string;
}

export interface Uplay {
  appId: string;
}

export interface GogGalaxy {
  productId: string;
}

export interface HumbleApp {
  gameName: string;
}

export interface RiotGames {
  appName: string;
}

// WebSocket payload interfaces

export interface ApplicationPayload {
  databaseId?: string;
  name: string;
  bundleId?: string;
  version?: number;
  installTime?: string;
  lastRunTime?: string;
  applicationPath: string;
  applicationFolder?: string;
  poster?: string;
  background?: string;
  themeMusic?: string;
  processId?: number;
  sessionId?: number;
  commandLine?: string;
  applicationId?: string;
  commands?: Command[];
  categoryColors?: CategoryColor[];
  isCustom?: boolean;
  posterUrl?: string;
  posterTitlePosition?: string;
  isDisabled?: boolean;
  isInstalled?: boolean;
  userPaths?: string[];
  posterPath?: string;
}