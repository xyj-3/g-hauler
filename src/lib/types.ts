// TypeScript interfaces matching the Rust structs

export interface GHUBApp {
  application_id: string;
  category_colors: CategoryColor[];
  commands: Command[];
  detection: Detection[];
  name: string;
  poster_title_position: string;
  poster_url: string;
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
  | { win_registry: WinRegistry }
  | { epic_games: EpicGames }
  | { osx_bundle: OsxBundle }
  | { uplay: Uplay }
  | { gog_galaxy: GogGalaxy }
  | { humble_app: HumbleApp }
  | { riot_games: RiotGames }
  | { glob: string }
  | any;

export interface SteamApp {
  app_id: string;
}

export interface WinRegistry {
  executable: string;
  registry_key: string;
  registry_path: string;
}

export interface EpicGames {
  app_name: string;
}

export interface OsxBundle {
  bundle_id: string;
  bundle_path: string;
}

export interface Uplay {
  app_id: string;
}

export interface GogGalaxy {
  product_id: string;
}

export interface HumbleApp {
  game_name: string;
}

export interface RiotGames {
  app_name: string;
}
