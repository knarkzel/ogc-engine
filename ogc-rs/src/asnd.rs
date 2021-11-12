//! The ``asnd`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the audio functions found in ``asnd.h``.
//!
//! Note: In order to use this library, you need to have the following linker flags
//! in your `powerpc-unknown-eabi.json` ("post-link-args" goes after "pre-link-args"):
//!
//! ```json
//! "post-link-args": {
//!     "gcc": [
//!         "-lasnd",
//!         "-logc"
//!     ]
//! },
//! ```

use crate::{OgcError, Result};
use alloc::boxed::Box;
use alloc::format;
use core::mem;

macro_rules! if_not {
    ($valid:ident => $error_output:expr, $var:ident $(,)*) => {
        if $var == ogc_sys::$valid as _ {
            Ok(())
        } else {
            Err(OgcError::Audio(format!($error_output, $var)))
        }
    };
}

/// Options to be passed when creating a new voice.
///
/// # Examples
///
/// Create `VoiceOptions` with voice slot 2 and format Mono16Bit:
///
/// ```rust
/// let options = VoiceOptions::new().voice(2).format(VoiceFormat::Mono16Bit);
/// ```
pub struct VoiceOptions {
    voice: u32,
    format: VoiceFormat,
    pitch: u32,
    delay: u32,
    volume_left: u8,
    volume_right: u8,
    callback: Option<Box<fn(i32) -> ()>>,
}

impl VoiceOptions {
    /// Create this struct with sensible default values.
    pub fn new() -> Self {
        Self {
            voice: 0,
            format: VoiceFormat::Stereo16Bit,
            pitch: 48000,
            delay: 0,
            volume_left: 255,
            volume_right: 255,
            callback: None,
        }
    }

    /// Voice slot to use for this sound. Valid values are `0..16` non-inclusive.
    pub fn voice(self, voice: u32) -> Self {
        assert!(voice < 16, "Voice index {} is >= 16", voice);
        Self { voice, ..self }
    }

    /// Format to use for this sound.
    pub fn format(self, format: VoiceFormat) -> Self {
        Self { format, ..self }
    }

    /// Frequency to use, in Hz.
    pub fn pitch(self, pitch: u32) -> Self {
        Self { pitch, ..self }
    }

    /// Delay to wait before playing, in milliseconds.
    pub fn delay(self, delay: u32) -> Self {
        Self { delay, ..self }
    }

    /// Voice volume of the left channel.
    pub fn volume_left(self, volume_left: u8) -> Self {
        Self {
            volume_left,
            ..self
        }
    }

    /// Voice volume of the right channel.
    pub fn volume_right(self, volume_right: u8) -> Self {
        Self {
            volume_right,
            ..self
        }
    }

    /// Optional callback function to use.
    pub fn callback(self, callback: Box<fn(i32) -> ()>) -> Self {
        let callback = Some(callback);
        Self { callback, ..self }
    }
}

/// Source voice format.
pub enum VoiceFormat {
    Mono8Bit,
    Mono16Bit,
    Mono16BitBe,
    Stereo8Bit,
    Stereo16Bit,
    Stereo16BitBe,
    Mono8BitU,
    Mono16BitLE,
    Stereo8BitU,
    Stereo16BitLe,
}

impl VoiceFormat {
    fn as_i32(self) -> i32 {
        match self {
            VoiceFormat::Mono8Bit => 0,
            VoiceFormat::Mono16Bit => 1,
            VoiceFormat::Mono16BitBe => 1,
            VoiceFormat::Stereo8Bit => 2,
            VoiceFormat::Stereo16Bit => 3,
            VoiceFormat::Stereo16BitBe => 3,
            VoiceFormat::Mono8BitU => 4,
            VoiceFormat::Mono16BitLE => 5,
            VoiceFormat::Stereo8BitU => 6,
            VoiceFormat::Stereo16BitLe => 7,
        }
    }
}

/// Represents the asnd service.
/// This service can only be created once!
///
/// # Minimal Example
///
/// ```rust
/// let asnd = Asnd::init();
/// let mut buffer = (0..255).cycle().take(32 * 32 * 32 * 32).collect::<Vec<_>>();
/// Asnd::set_voice(VoiceOptions::new(), &mut buffer).unwrap();
/// Asnd::pause(false);
/// ```
pub struct Asnd;

/// Implementation of the asnd service.
#[allow(unused_unsafe)]
impl Asnd {
    /// Initializes the asnd lib and fixes the hardware sample rate to 48000hz.
    pub fn init() -> Self {
        unsafe { ogc_sys::ASND_Init() };
        Self
    }

    /// De-initializes the asnd lib.
    pub fn end() {
        unsafe { ogc_sys::ASND_End() };
    }

    /// Pauses if true and resumes if false.
    pub fn pause(should_pause: bool) {
        unsafe { ogc_sys::ASND_Pause(should_pause as _) };
    }

    /// Returns true if paused, false if not paused.
    pub fn is_paused() -> bool {
        unsafe { ogc_sys::ASND_Is_Paused() > 0 }
    }

    /// Returns the global time in milliseconds. Time is updated from the IRQ.
    pub fn get_time() -> u32 {
        unsafe { ogc_sys::ASND_GetTime() }
    }

    /// Returns the global sample counter. Can be used to implement timers with high precision.
    pub fn get_sample_counter() -> u32 {
        unsafe { ogc_sys::ASND_GetSampleCounter() }
    }

    /// Returns the samples sent from the IRQ in one tick.
    pub fn get_samples_per_tick() -> u32 {
        unsafe { ogc_sys::ASND_GetSamplesPerTick() }
    }

    /// Sets the global time, in milliseconds.
    pub fn set_time(time: u32) {
        unsafe { ogc_sys::ASND_SetTime(time) };
    }

    /// Sets a global callback for general purposes. It is called by the IRQ.
    pub fn set_callback<F>(callback: Box<F>)
    where
        F: Fn() -> (),
    {
        // TODO: Check if this implementation can be changed.
        let ptr = Box::into_raw(callback);

        unsafe {
            let code: extern "C" fn() = mem::transmute(ptr);
            ogc_sys::ASND_SetCallback(Some(code));
        }
    }

    /// Returs the current audio rate. Default is 48000hz.
    pub fn get_audio_rate() -> i32 {
        unsafe { ogc_sys::ASND_GetAudioRate() }
    }

    /// Sets a PCM voice to play. This function stops one previous voice. Use
    /// `Asnd::status_voice()` to test status. The voices are played in 16-bit stereo,
    /// regardless of source format. The buffer MUST be aligned and padded to 32 bytes.
    pub fn set_voice(options: VoiceOptions, sound_buffer: &mut [u8]) -> Result<()> {
        let callback = options.callback.map(|f| {
            let ptr = Box::into_raw(f);
            let code: unsafe extern "C" fn(i32) = unsafe { mem::transmute(ptr) };
            code
        });

        let err = unsafe {
            ogc_sys::ASND_SetVoice(
                options.voice as _,
                options.format.as_i32(),
                options.pitch as _,
                options.delay as _,
                sound_buffer.as_mut_ptr() as *mut _,
                sound_buffer.len() as _,
                options.volume_left as _,
                options.volume_right as _,
                callback,
            )
        };

        if_not!(SND_OK => "Asnd::set_voice() failed with error {}!", err)
    }

    /// Sets a PCM voice to play infinitely. See `Asnd::set_voice()` as it is largely identical.
    /// The buffer MUST be aligned and padded to 32 bytes.
    pub fn set_infinite_voice(options: VoiceOptions, sound_buffer: &mut [u8]) -> Result<()> {
        let err = unsafe {
            ogc_sys::ASND_SetInfiniteVoice(
                options.voice as _,
                options.format.as_i32(),
                options.pitch as _,
                options.delay as _,
                sound_buffer.as_mut_ptr() as *mut _,
                sound_buffer.len() as _,
                options.volume_left as _,
                options.volume_right as _,
            )
        };

        if_not!(SND_OK => "Asnd::set_infinite_voice() failed with error {}", err)
    }

    /// Adds a PCM voice to play from the second buffer. Sound buffer must be 32-byte
    /// aligned and have same sample format as first buffer. This must only be called after
    /// `Asnd::set_voice()`, which must return `Ok()`.
    /// The buffer MUST be aligned and padded to 32 bytes.
    pub fn add_voice(voice: u32, sound_buffer: &mut [u8]) -> Result<()> {
        assert!(voice < 16, "Voice index {} is >= 16", voice);

        let err = unsafe {
            ogc_sys::ASND_AddVoice(
                voice as _,
                sound_buffer.as_mut_ptr() as *mut _,
                sound_buffer.len() as _,
            )
        };

        if_not!(SND_OK => "Asnd::add_voice() failed with error {}", err)
    }

    /// Stops the selected voice. If the voice is used in song mode, you need to
    /// assign the samples with `Asnd::set_song_sample_voice()`.
    pub fn stop_voice(voice: u32) -> Result<()> {
        assert!(voice < 16, "Voice index {} is >= 16", voice);
        let err = unsafe { ogc_sys::ASND_StopVoice(voice as _) };
        if_not!(SND_OK => "Asnd::stop_voice() failed with error {}", err)
    }

    /// Pauses the selected voice. Can also be used to resume voice.
    pub fn pause_voice(voice: u32, pause: bool) -> Result<()> {
        assert!(voice < 16, "Voice index {} is >= 16", voice);
        let err = unsafe { ogc_sys::ASND_PauseVoice(voice as _, pause as _) };
        if_not!(SND_OK => "Asnd::pause_voice() failed with error {}", err)
    }

    /// Returns the state of the selected voice.
    pub fn status_voice(voice: u32) -> Result<()> {
        assert!(voice < 16, "Voice index {} is >= 16", voice);
        let err = unsafe { ogc_sys::ASND_StatusVoice(voice as _) };
        if_not!(SND_WORKING => "Asnd::status_voice() failed with error {}", err)
    }

    /// Returns the first unused voice. Fails if no voices are available.
    pub fn get_first_unused_voice() -> Result<u32> {
        let err = unsafe { ogc_sys::ASND_GetFirstUnusedVoice() };
        match err {
            x if x < 16 && x >= 0 => Ok(x as _),
            _ => Err(OgcError::Audio(format!(
                "Asnd::get_first_unused_voice() failed with error {}",
                err
            ))),
        }
    }

    /// Changes the voice-pitch in real time. This function can be used to
    /// create audio effects such as Doppler effect simulation.
    pub fn change_pitch_voice(voice: u32, pitch: u32) -> Result<()> {
        assert!(voice < 16, "Voice index {} is >= 16", voice);
        let err = unsafe { ogc_sys::ASND_ChangePitchVoice(voice as _, pitch as _) };
        if_not!(SND_OK => "Asnd::change_pitch_voice() failed with error {}", err)
    }

    /// Changes the voice volume in real time. This function can be used to create
    /// audio effects like distance attenuation.
    pub fn change_volume_voice(voice: u32, volume_left: u8, volume_right: u8) -> Result<()> {
        assert!(voice < 16, "Voice index {} is >= 16", voice);
        let err = unsafe {
            ogc_sys::ASND_ChangeVolumeVoice(voice as _, volume_left as _, volume_right as _)
        };
        if_not!(SND_OK => "Asnd::change_volume_voice() failed with error {}", err)
    }

    /// Returns the voice tick counter. This value represents the number of ticks
    /// since this voice started to play, sans delay time. If the lib is initialized with
    /// `INIT_RATE=48000`, a return value of 24000 is equal to 0.5 seconds.
    pub fn get_tick_counter_voice(voice: u32) -> u32 {
        assert!(voice < 16, "Voice index {} is >= 16", voice);
        unsafe { ogc_sys::ASND_GetTickCounterVoice(voice as _) }
    }

    /// Returns the voice playback time. This value represents the time in milliseconds
    /// since this voice started playing.
    pub fn get_timer_voice(voice: u32) -> u32 {
        assert!(voice < 16, "Voice index {} is >= 16", voice);
        unsafe { ogc_sys::ASND_GetTimerVoice(voice as _) }
    }

    /// Tests if a pointer is in use by a voice as a buffer.
    /// This must be the same pointer sent to `Asnd::add_voice()` or `Asnd::set_voice()`.
    /// Returns 0 if the pointer is unused.
    /// Returns 1 if the pointer is used as a buffer.
    /// Returns `ogc_sys::SND_INVALID` if invalid.
    pub fn test_pointer<T>(voice: u32, pointer: *mut T) -> i32 {
        assert!(voice < 16, "Voice index {} is >= 16", voice);
        unsafe { ogc_sys::ASND_TestPointer(voice as _, pointer as *mut _) }
    }

    /// Tests to determine if the voice is ready to receive a new buffer sample
    /// with `Asnd::add_voice()`. Returns true if voice is ready.
    pub fn test_voice_buffer_ready(voice: u32) -> bool {
        assert!(voice < 16, "Voice index {} is >= 16", voice);
        unsafe { ogc_sys::ASND_TestVoiceBufferReady(voice as _) > 0 }
    }

    /// Returns the DSP usage, in percent `(0..=100)`.
    pub fn get_dsp_percent_use() -> u32 {
        unsafe { ogc_sys::ASND_GetDSP_PercentUse() }
    }

    /// Returns DSP process time, in nano seconds.
    pub fn get_dsp_process_time() -> u32 {
        unsafe { ogc_sys::ASND_GetDSP_ProcessTime() }
    }
}
