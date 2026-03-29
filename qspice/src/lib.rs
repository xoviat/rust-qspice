pub use qspice_macros::{main, max, trunc};

use std::ffi::{CStr, CString};
use std::io::Write;
use std::str::Utf8Error;

mod ffi {
    use core::ffi::{c_char, c_void};
    use core::ptr;

    #[unsafe(no_mangle)]
    pub static mut Display: Option<unsafe extern "C" fn(format: *const i8, ...)> = None;
    #[unsafe(no_mangle)]
    pub static mut EXIT: Option<unsafe extern "C" fn(format: *const i8, ...)> = None;
    #[unsafe(no_mangle)]
    pub static mut DegreesC: *const f64 = ptr::null(); // pointer to current circuit temperature
    #[unsafe(no_mangle)]
    pub static mut StepNumber: *const i32 = ptr::null(); // pointer to current step number
    #[unsafe(no_mangle)]
    pub static mut NumberSteps: *const i32 = ptr::null(); // pointer to estimated number of steps
    #[unsafe(no_mangle)]
    pub static mut InstanceName: *const *const c_char = ptr::null(); // pointer to address of instance name
    #[unsafe(no_mangle)]
    pub static mut QUX: *const c_char = ptr::null(); // path to QUX.exe
    #[unsafe(no_mangle)]
    pub static mut ForKeeps: *const bool = ptr::null(); // pointer to whether being evaluated non-hypothetically
    #[unsafe(no_mangle)]
    pub static mut HoldICs: *const bool = ptr::null(); // pointer to whether instance initial conditions are being held
    #[unsafe(no_mangle)]
    pub static mut GUI_HWND: *const c_void = ptr::null(); // pointer to Window handle of QUX.exe
    #[unsafe(no_mangle)]
    pub static mut CKTtime: *const f64 = ptr::null();
    #[unsafe(no_mangle)]
    pub static mut CKTdelta: *const f64 = ptr::null();
    #[unsafe(no_mangle)]
    pub static mut IntegrationOrder: *const i32 = ptr::null();
    #[unsafe(no_mangle)]
    pub static mut InstallDirectory: *const c_char = ptr::null();
}

pub struct QSpice {
    _private: (),
}

impl QSpice {
    pub const unsafe fn new() -> Self {
        Self { _private: () }
    }

    /// current circuit temperature
    pub fn temperature(&self) -> f64 {
        unsafe { *ffi::DegreesC }
    }

    /// current step number
    pub fn step_number(&self) -> i32 {
        unsafe { *ffi::StepNumber }
    }

    /// estimated number of steps
    pub fn number_steps(&self) -> i32 {
        unsafe { *ffi::NumberSteps }
    }

    /// instance name
    pub fn instance_name(&self) -> Result<String, Utf8Error> {
        unsafe { Ok(CStr::from_ptr(*ffi::InstanceName).to_str()?.to_string()) }
    }

    /// path to QUX.exe
    pub fn qux_path(&self) -> Result<String, Utf8Error> {
        unsafe { Ok(CStr::from_ptr(ffi::QUX).to_str()?.to_string()) }
    }

    /// whether being evaluated non-hypothetically
    pub fn for_keeps(&self) -> bool {
        unsafe { *ffi::ForKeeps }
    }

    /// whether instance initial conditions are being held
    pub fn hold_ics(&self) -> bool {
        unsafe { *ffi::HoldICs }
    }
}

impl Write for QSpice {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // This way ensures that format specifiers in the string are ignored
        let sp = CString::new("%s")?;
        let st = CString::new(buf)?;

        unsafe {
            let fun = ffi::Display.unwrap();
            fun(sp.as_ptr(), st.as_ptr());
        }

        Ok(st.count_bytes())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
