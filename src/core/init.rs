//! Provides initialization code for drivers used by oxcable

#![experimental]

extern crate portaudio;
extern crate portmidi;


/// Set to `true` on initialization and `false` on termination, used to remember
/// the state of our drivers.
static mut INITIALIZED: bool = false;


/// Attempts to initialize our drivers so that IO can be performed.
///
/// On failure, `Err` is returned along with a failure message.
pub fn initialize() -> Result<(), &'static str> {
    unsafe {
        if INITIALIZED {
            return Result::Ok(());
        }
    }

    if portaudio::pa::initialize().is_err() {
        return Result::Err("failed to initialize portaudio");
    }
    if portmidi::initialize().is_err() {
        return Result::Err("failed to initialize portmidi");
    }

    unsafe { INITIALIZED = true; }
    Result::Ok(())
}


/// Returns `true` if we have initialized our drivers.
pub fn is_initialized() -> bool {
    unsafe { INITIALIZED }
}


/// Attempts to cleanly terminate our drivers after IO has finished.
///
/// On failure, `Err` is returned along with a failure message.
pub fn terminate() -> Result<(), &'static str> {
    unsafe {
        if !INITIALIZED {
            return Result::Ok(());
        }
    }

    if portaudio::pa::terminate().is_err() {
        return Result::Err("failed to terminate portaudio");
    }
    if portmidi::terminate().is_err() {
        return Result::Err("failed to terminate portmidi");
    }

    unsafe { INITIALIZED = false; }
    Result::Ok(())
}
