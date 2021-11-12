//! The ``mp3player`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the mp3-related functions found in ``mp3player.h``.
//!
//! Note: In order to use this library, you need to have the following linker flags
//! in your `powerpc-unknown-eabi.json` ("post-link-args" goes after "pre-link-args"):
//!
//! ```json
//! "post-link-args": {
//!     "gcc": [
//!         "-lasnd",
//!         "-logc",
//!         "-lmad"
//!     ]
//! },
//! ```

/// Represents the mp3player service.
/// This service can only be created once!
///
/// # Minimal Example
///
/// ```rust
/// const MUSIC: &[u8] = include_bytes!("../sample.mp3");
/// Asnd::init();
/// Mp3Player::init();
/// Mp3Player::play_buffer(MUSIC);
/// ```
pub struct Mp3Player;

impl Mp3Player {
    pub fn init() {
        unsafe { ogc_sys::MP3Player_Init() };
    }

    pub fn play_buffer(sound_buffer: &[u8]) {
        unsafe {
            ogc_sys::MP3Player_PlayBuffer(
                sound_buffer.as_ptr() as *const _,
                sound_buffer.len() as _,
                None,
            )
        };
    }

    pub fn volume(volume: u8) {
        unsafe { ogc_sys::MP3Player_Volume(volume as _) };
    }

    pub fn is_playing() -> bool {
        unsafe { ogc_sys::MP3Player_IsPlaying() }
    }

    pub fn stop() {
        unsafe { ogc_sys::MP3Player_Stop() };
    }
}
