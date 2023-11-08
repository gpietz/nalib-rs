use std::os::raw::c_void;

#[link(name = "NALib", kind = "static")]
extern "C" {
    pub(crate) fn naStartRuntime();
    pub(crate) fn naStopRuntime();
    pub(crate) static na_Runtime: *mut c_void;
}
