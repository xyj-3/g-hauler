#[derive(serde::Serialize)]
pub struct PathValidationResult {
    pub data_path_exists: bool,
    pub applications_json_exists: bool,
    pub current_json_exists: bool,
    pub version_json_exists: bool,
    pub build_id: Option<String>,
    pub images_dir_exists: bool,
}