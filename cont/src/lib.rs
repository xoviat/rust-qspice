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

#[derive(Default)]
pub struct sCONT {
    // declare the structure here
}

#[qspice::main]
fn cont(
    cont: &mut sCONT,
    t: f64,
    data: (
        f32,
        f32,
        f32,
        f32,
        f32,
        &mut bool,
        &mut f32,
        &mut f32,
        &mut f32,
        &mut f32,
        &mut f32,
        &mut f32,
        &mut f32,
        &mut f32,
        &mut f32,
    ),
) {
}

#[qspice::max]
fn max_ext_step_size(inst: &mut sCONT, t: f64) -> f64 {
    1e308 // implement a good choice of max timestep size that depends on struct sCONT
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Trunc(inst: *mut sCONT, t: f64, data: *mut __u_data, timestep: *mut f64) {
    unsafe {
        let vbulk: f32 = (*data.add(0)).f; // input
        let v_ab: f32 = (*data.add(1)).f; // input
        let v_bc: f32 = (*data.add(2)).f; // input
        let i_1: f32 = (*data.add(3)).f; // input
        let i_2: f32 = (*data.add(4)).f; // input

        let en: &mut bool = &mut (*data.add(5)).b; // output
        let a1: &mut f32 = &mut (*data.add(6)).f; // output
        let a3: &mut f32 = &mut (*data.add(7)).f; // output
        let a5: &mut f32 = &mut (*data.add(8)).f; // output
        let vd: &mut f32 = &mut (*data.add(9)).f; // output
        let vq: &mut f32 = &mut (*data.add(10)).f; // output
        let vd_ref: &mut f32 = &mut (*data.add(11)).f; // output
        let vq_ref: &mut f32 = &mut (*data.add(12)).f; // output
        let va_ref: &mut f32 = &mut (*data.add(13)).f; // output
        let vb_ref: &mut f32 = &mut (*data.add(14)).f; // output

        trunc_inner(
            &mut *inst,
            t,
            (
                vbulk, v_ab, v_bc, i_1, i_2, en, a1, a3, a5, vd, vq, vd_ref, vq_ref, va_ref, vb_ref,
            ),
            &mut *timestep,
        );
    }
}

fn trunc_inner(
    cont: &mut sCONT,
    t: f64,
    data: (
        f32,
        f32,
        f32,
        f32,
        f32,
        &mut bool,
        &mut f32,
        &mut f32,
        &mut f32,
        &mut f32,
        &mut f32,
        &mut f32,
        &mut f32,
        &mut f32,
        &mut f32,
    ),
    timestep: &mut f64,
) {
}
