#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]

use core::ffi::{c_char, c_void};
use core::ptr;

#[unsafe(no_mangle)]
pub static mut Display: *mut c_void = ptr::null_mut(); // works like printf()
#[unsafe(no_mangle)]
pub static mut EXIT: *mut c_void = ptr::null_mut(); // print message like printf() but exit(0) afterward
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
#[unsafe(no_mangle)]
pub static mut EngAtof: *mut c_void = ptr::null_mut();
#[unsafe(no_mangle)]
pub static mut BinaryFormat: *mut c_void = ptr::null_mut(); // BinaryFormat(0x1C) returns "0b00011100"
#[unsafe(no_mangle)]
pub static mut EngFormat: *mut c_void = ptr::null_mut(); // EngFormat(1e-6, "s", 6) returns "1µs"
#[unsafe(no_mangle)]
pub static mut DFFT: *mut c_void = ptr::null_mut(); // Discrete Fast Fourier Transform
#[unsafe(no_mangle)]
pub static mut bzero: *mut c_void = ptr::null_mut();

#[repr(C)]
pub union uData {
    pub b: bool,
    pub c: i8,
    pub uc: u8,
    pub s: i16,
    pub us: u16,
    pub i: i32,
    pub ui: u32,
    pub f: f32,
    pub d: f64,
    pub i64: i64,
    pub ui64: u64,
    pub str: *mut c_char,
    pub bytes: *mut u8,
}

// int DllMain() must exist and return 1 for a process to load the .DLL
// See https://docs.microsoft.com/en-us/windows/win32/dlls/dllmain for more information.
#[unsafe(no_mangle)]
pub extern "system" fn DllMain(_module: *mut c_void, _reason: u32, _reserved: *mut c_void) -> i32 {
    1
}

// #undef pin names lest they collide with names in any header file(s) you might include.

#[repr(C)]
pub struct sCONT {
    // declare the structure here
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cont(opaque: *mut *mut sCONT, t: f64, data: *mut uData) {
    let VBULK: f32 = (*data.add(0)).f; // input
    let V_AB: f32 = (*data.add(1)).f; // input
    let V_BC: f32 = (*data.add(2)).f; // input
    let I_1: f32 = (*data.add(3)).f; // input
    let I_2: f32 = (*data.add(4)).f; // input

    let EN: &mut bool = &mut (*data.add(5)).b; // output
    let A1: &mut f32 = &mut (*data.add(6)).f; // output
    let A3: &mut f32 = &mut (*data.add(7)).f; // output
    let A5: &mut f32 = &mut (*data.add(8)).f; // output
    let VD: &mut f32 = &mut (*data.add(9)).f; // output
    let VQ: &mut f32 = &mut (*data.add(10)).f; // output
    let VD_REF: &mut f32 = &mut (*data.add(11)).f; // output
    let VQ_REF: &mut f32 = &mut (*data.add(12)).f; // output
    let VA_REF: &mut f32 = &mut (*data.add(13)).f; // output
    let VB_REF: &mut f32 = &mut (*data.add(14)).f; // output

    if (*opaque).is_null() {}

    // Implement module evaluation code here:
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn MaxExtStepSize(_inst: *mut sCONT, _t: f64) -> f64 {
    1e308 // implement a good choice of max timestep size that depends on struct sCONT
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Trunc(inst: *mut sCONT, t: f64, data: *mut uData, timestep: *mut f64) {
    // limit the timestep to a tolerance if the circuit causes a change in struct sCONT
    let ttol: f64 = 1e-9; // 1ns default tolerance
    if *timestep > ttol {
        // lower timestep if possible
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Destroy(inst: *mut sCONT) {}
