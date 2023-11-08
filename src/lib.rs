mod memory;

pub use self::memory::*;

pub fn start_runtime() {
    unsafe {
        naStartRuntime();
    }
}

pub fn stop_runtime() {
    unsafe {
        naStopRuntime();
    }
}

pub fn is_runtime_running() -> bool {
    unsafe { !na_Runtime.is_null() }
}
