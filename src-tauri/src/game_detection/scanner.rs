use super::models::*;
use super::platforms::*;

/// Helper macro to scan a platform and handle results
macro_rules! scan_platform {
    ($result:expr, $detector:expr, $platform_name:expr) => {
        match $detector.scan_games().await {
            Ok(games) => $result.add_games($platform_name.to_string(), games),
            Err(e) => $result.mark_failed(
                $platform_name.to_string(),
                format!("{} scan error: {}", $platform_name, e),
            ),
        }
    };
}

/// Main game scanner that orchestrates detection across different platforms
pub struct GameScanner {
    options: ScanOptions,
}

impl GameScanner {
    pub fn new(options: ScanOptions) -> Self {
        Self { options }
    }

    /// Scan for all installed games based on the configured options
    pub async fn scan_installed_games(&self) -> Result<GameScanResult, String> {
        let start_time = std::time::Instant::now();
        let mut result = GameScanResult::new();

        if self.options.scan_steam {
            scan_platform!(result, SteamDetector::new(), GamePlatform::STEAM_NAME);
        }

        if self.options.scan_epic_games {
            scan_platform!(result, EpicDetector::new(), GamePlatform::EPIC_GAMES_NAME);
        }

        if self.options.scan_uplay {
            scan_platform!(result, UplayDetector::new(), GamePlatform::UPLAY_NAME);
        }

        if self.options.scan_gog_galaxy {
            scan_platform!(result, GogDetector::new(), GamePlatform::GOG_GALAXY_NAME);
        }

        if self.options.scan_riot_games {
            scan_platform!(result, RiotDetector::new(), GamePlatform::RIOT_GAMES_NAME);
        }

        if self.options.scan_win_registry {
            scan_platform!(result, RegistryDetector::new(), GamePlatform::WIN_REGISTRY_NAME);
        }

        if self.options.scan_osx_bundle {
            scan_platform!(result, OsxDetector::new(), GamePlatform::OSX_BUNDLE_NAME);
        }

        let scan_duration_ms = start_time.elapsed().as_millis() as u64;

        Ok(result.finalize(scan_duration_ms))
    }
}

impl Default for GameScanner {
    fn default() -> Self {
        Self::new(ScanOptions::default())
    }
}
