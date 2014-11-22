//! Initialization code for drivers used by oxcable

#![experimental]

extern crate portaudio;
extern crate portmidi;

static mut INITIALIZED: bool = false;

pub fn initialize() -> Result<(), &'static str> {
    unsafe {
        if INITIALIZED {
            return Result::Ok(());
        }
    }

    if portaudio::pa::initialize().is_err() {
        return Result::Err("failed to initialize portaudio");
    }
    if portmidi::midi::initialize() != portmidi::midi::PmError::PmNoError {
        return Result::Err("failed to initialize portmidi");
    }

    unsafe { INITIALIZED = true; }
    Result::Ok(())
}

pub fn is_initialized() -> bool {
    unsafe { INITIALIZED }
}

pub fn terminate() -> Result<(), &'static str> {
    unsafe {
        if !INITIALIZED {
            return Result::Ok(());
        }
    }

    if portaudio::pa::terminate().is_err() {
        return Result::Err("failed to terminate portaudio");
    }
    if portmidi::midi::terminate() != portmidi::midi::PmError::PmNoError {
        return Result::Err("failed to terminate portmidi");
    }

    unsafe { INITIALIZED = false; }
    Result::Ok(())
}
