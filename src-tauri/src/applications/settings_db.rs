use crate::core::constants::LGHUB_SETTINGS_DB_PATH;
use crate::core::state::AppState;
use crate::applications::models::{ApplicationsData, GHUBApp};
use chrono::Utc;
use rusqlite::Connection;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};

fn expand_env_path(path: &str) -> PathBuf {
    if path.contains("%LOCALAPPDATA%") {
        if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            PathBuf::from(path.replace("%LOCALAPPDATA%", &local_app_data))
        } else {
            PathBuf::from(path)
        }
    } else {
        PathBuf::from(path)
    }
}

fn get_settings_db_path() -> PathBuf {
    expand_env_path(LGHUB_SETTINGS_DB_PATH)
}

fn setup_connection(db_path: &Path) -> Result<Connection, String> {
    let conn = Connection::open(db_path).map_err(|e| format!("Failed to open database: {}", e))?;

    // Check if WAL mode files exist (newer LGHUB versions)
    let wal_exists = db_path.with_extension("db-wal").exists();
    let shm_exists = db_path.with_extension("db-shm").exists();

    if wal_exists && shm_exists {
        conn.execute("PRAGMA journal_mode=WAL", [])
            .map_err(|e| format!("Failed to set WAL mode: {}", e))?;
    } else {
        conn.execute("PRAGMA journal_mode=DELETE", [])
            .map_err(|e| format!("Failed to set DELETE mode: {}", e))?;
    }

    Ok(conn)
}

fn get_latest_data_id(conn: &Connection) -> Result<i64, String> {
    let mut stmt = conn
        .prepare("SELECT MAX(_id) FROM DATA")
        .map_err(|e| format!("Failed to prepare query for latest ID: {}", e))?;

    let latest_id: i64 = stmt
        .query_row([], |row| row.get(0))
        .map_err(|e| format!("Failed to get latest ID: {}", e))?;

    Ok(latest_id)
}

fn create_backup_path(original_path: &Path) -> PathBuf {
    let mut backup_path = original_path.to_path_buf();
    if let Some(extension) = original_path.extension() {
        backup_path.set_extension(format!("{}.backup", extension.to_string_lossy()));
    } else {
        backup_path.set_extension("backup");
    }
    backup_path
}

pub fn backup_sqlite_on_startup() -> Result<(), String> {
    let db_path = get_settings_db_path();

    if !db_path.exists() {
        println!(
            "Settings database not found at: {}, skipping backup",
            db_path.display()
        );
        return Ok(());
    }

    let backup_path = create_backup_path(&db_path);

    fs::copy(&db_path, &backup_path)
        .map_err(|e| format!("Failed to create backup of settings.db: {}", e))?;

    println!("Successfully created backup at: {}", backup_path.display());
    Ok(())
}

pub fn load_and_store_sqlite_data(app_handle: &AppHandle) -> Result<ApplicationsData, String> {
    let db_path = get_settings_db_path();

    if !db_path.exists() {
        return Err(format!(
            "Settings database not found at: {}",
            db_path.display()
        ));
    }

    let conn = setup_connection(&db_path)?;
    let latest_id = get_latest_data_id(&conn)?;

    // Get the latest settings blob from the DATA table
    let mut stmt = conn
        .prepare("SELECT _id, FILE FROM DATA WHERE _id = ?")
        .map_err(|e| format!("Failed to prepare blob query: {}", e))?;

    let blob_data: Vec<u8> = stmt
        .query_row([latest_id], |row| row.get::<_, Vec<u8>>(1))
        .map_err(|e| format!("Failed to fetch blob data: {}", e))?;

    // Convert blob to string and parse as JSON
    let json_str = String::from_utf8(blob_data)
        .map_err(|e| format!("Failed to convert blob to UTF-8: {}", e))?;

    // Parse the JSON as ApplicationsData structure
    let applications_data: ApplicationsData = serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed to parse JSON from database: {}", e))?;

    // Store the loaded data in the app state
    let state: tauri::State<AppState> = app_handle.state();
    let mut db_data = state
        .settings_db_data
        .lock()
        .map_err(|e| format!("Failed to acquire lock on settings_db_data: {}", e))?;
    *db_data = Some(applications_data.clone());

    // Also store applications in the applications field (for backward compatibility)
    let mut apps = state
        .applications
        .lock()
        .map_err(|e| format!("Failed to acquire lock on applications: {}", e))?;
    *apps = applications_data.applications.clone();

    println!(
        "Successfully loaded {} applications from SQLite database",
        applications_data.applications.len()
    );
    Ok(applications_data)
}

pub fn load_from_sqlite(app_handle: &AppHandle) -> Result<Vec<GHUBApp>, String> {
    let db_path = get_settings_db_path();

    if !db_path.exists() {
        return Err(format!(
            "Settings database not found at: {}",
            db_path.display()
        ));
    }

    let conn = setup_connection(&db_path)?;
    let latest_id = get_latest_data_id(&conn)?;

    // Get the latest settings blob from the DATA table
    let mut stmt = conn
        .prepare("SELECT _id, FILE FROM DATA WHERE _id = ?")
        .map_err(|e| format!("Failed to prepare blob query: {}", e))?;

    let blob_data: Vec<u8> = stmt
        .query_row([latest_id], |row| row.get::<_, Vec<u8>>(1))
        .map_err(|e| format!("Failed to fetch blob data: {}", e))?;

    // Convert blob to string and parse as JSON
    let json_str = String::from_utf8(blob_data)
        .map_err(|e| format!("Failed to convert blob to UTF-8: {}", e))?;

    // Parse the JSON as ApplicationsData structure
    let applications_data: ApplicationsData = serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed to parse JSON from database: {}", e))?;

    // Store the loaded applications in the app state
    let state: tauri::State<AppState> = app_handle.state();
    let mut apps = state
        .applications
        .lock()
        .map_err(|e| format!("Failed to acquire lock on applications: {}", e))?;
    *apps = applications_data.applications.clone();

    println!(
        "Successfully loaded {} applications from SQLite database",
        applications_data.applications.len()
    );
    Ok(applications_data.applications)
}

pub fn write_to_sqlite(app_handle: &AppHandle) -> Result<(), String> {
    let db_path = get_settings_db_path();

    if !db_path.exists() {
        return Err(format!(
            "Settings database not found at: {}. Cannot write to non-existent database.",
            db_path.display()
        ));
    }

    // Get applications from app state
    let state: tauri::State<AppState> = app_handle.state();
    let apps = state
        .applications
        .lock()
        .map_err(|e| format!("Failed to acquire lock on applications: {}", e))?;

    // Create ApplicationsData structure
    let applications_data = ApplicationsData {
        applications: apps.clone(),
    };

    // Serialize to JSON
    let json_str = serde_json::to_string(&applications_data)
        .map_err(|e| format!("Failed to serialize applications to JSON: {}", e))?;

    // Convert to bytes for BLOB storage
    let json_bytes = json_str.as_bytes();

    let conn = setup_connection(&db_path)?;
    let latest_id = get_latest_data_id(&conn)?;
    let new_id = latest_id + 1;

    // Insert new record with updated data
    let current_time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let mut stmt = conn
        .prepare("INSERT INTO DATA (_id, _date_created, FILE) VALUES (?1, ?2, ?3)")
        .map_err(|e| format!("Failed to prepare insert statement: {}", e))?;

    stmt.execute((new_id, &current_time, json_bytes))
        .map_err(|e| format!("Failed to insert new data record: {}", e))?;

    println!(
        "Successfully saved {} applications to SQLite database with ID: {}",
        apps.len(),
        new_id
    );
    Ok(())
}

#[tauri::command]
pub async fn load_applications_from_sqlite(app_handle: AppHandle) -> Result<Vec<GHUBApp>, String> {
    load_from_sqlite(&app_handle)
}

#[tauri::command]
pub async fn save_applications_to_sqlite(app_handle: AppHandle) -> Result<(), String> {
    write_to_sqlite(&app_handle)
}
