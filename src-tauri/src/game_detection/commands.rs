use super::models::{GameScanResult, ScanOptions};
use super::scanner::GameScanner;
use tauri::AppHandle;

/// Scan for all locally installed games
#[tauri::command]
pub async fn scan_installed_games(
    _app_handle: AppHandle,
    options: Option<ScanOptions>,
) -> Result<GameScanResult, String> {
    let scan_options = options.unwrap_or_default();
    let scanner = GameScanner::new(scan_options);

    scanner.scan_installed_games().await
}

/// Quick scan with default options for commonly installed platforms
#[tauri::command]
pub async fn quick_scan_games(_app_handle: AppHandle) -> Result<GameScanResult, String> {
    let scanner = GameScanner::new(ScanOptions::quick());
    scanner.scan_installed_games().await
}

/// Full comprehensive scan with all options enabled
#[tauri::command]
pub async fn full_scan_games(_app_handle: AppHandle) -> Result<GameScanResult, String> {
    let scanner = GameScanner::new(ScanOptions::all());
    scanner.scan_installed_games().await
}
