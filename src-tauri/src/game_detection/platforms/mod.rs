pub mod steam;
pub mod epic;
pub mod win_registry;
pub mod uplay;
pub mod gog;
pub mod riot;
pub mod osx;
pub mod ea_app;

pub use steam::SteamDetector;
pub use epic::EpicDetector;
pub use win_registry::RegistryDetector;
pub use uplay::UplayDetector;
pub use gog::GogDetector;
pub use riot::RiotDetector;
pub use osx::OsxDetector;
pub use ea_app::EaAppDetector;
