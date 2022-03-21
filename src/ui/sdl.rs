//! SDL-specific parts of the UI.

pub mod event;
pub mod manager;

pub use manager::Manager;

/// Framerate limiter to prevent the UI from repainting itself overly quickly.
pub struct Limiter(sdl2::gfx::framerate::FPSManager);

impl Limiter {
    /// Constructs a new limiter with the given framerate (Hz).
    ///
    /// # Errors
    ///
    /// Fails if we can't set the framerate.
    pub fn new(hz: u32) -> super::Result<Self> {
        let mut manager = sdl2::gfx::framerate::FPSManager::new();
        manager.set_framerate(hz).map_err(super::Error::Init)?;
        Ok(Self(manager))
    }

    /// Delays execution so as to maintain the framerate.
    pub fn delay(&mut self) {
        self.0.delay();
    }
}
