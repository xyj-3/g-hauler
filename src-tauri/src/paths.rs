pub struct PlatformPath {
    pub windows: &'static str,
    // pub macos: &'static str,
}

impl PlatformPath {
    pub fn current(&self) -> &'static str {
        if cfg!(target_os = "windows") {
            self.windows
        } else {
            panic!(
                "Unsupported operating system: this application only supports Windows and MacOS."
            );
        }
    }
}

pub const LGHUB_DEFAULT_PATH: PlatformPath = PlatformPath {
    windows: r"C:\Program Files\LGHUB",
    // macos: "/Applications/LGHUB.app",
};

pub const LGHUB_DATA_PATH: PlatformPath = PlatformPath {
    windows: r"C:\ProgramData\LGHUB",
    // macos: "/Applications/LGHUB.app",
};
