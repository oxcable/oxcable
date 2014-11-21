//! Initialization code for drivers used by oxcable

#![experimental]

extern crate portaudio;

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

    unsafe { INITIALIZED = false; }
    Result::Ok(())
}
