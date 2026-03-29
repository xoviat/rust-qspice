pub use qspice_macros::{main, max, trunc};

use std::ffi::CString;
use std::io::Write;

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

    pub fn temperature(&self) -> f64 {
        unsafe { *ffi::DegreesC }
    }

    pub fn step_number(&self) -> i32 {
        unsafe { *ffi::StepNumber }
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
