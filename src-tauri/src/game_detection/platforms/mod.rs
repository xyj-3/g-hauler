pub mod steam;
pub mod epic;
pub mod registry;
pub mod uplay;
pub mod gog;
pub mod riot;
pub mod humble;
pub mod osx;

pub use steam::SteamDetector;
pub use epic::EpicDetector;
pub use registry::RegistryDetector;
pub use uplay::UplayDetector;
pub use gog::GogDetector;
pub use riot::RiotDetector;
pub use humble::HumbleDetector;
pub use osx::OsxDetector;
