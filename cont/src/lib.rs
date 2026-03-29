use std::io::Write;

use qspice::QSpice;

#[derive(Default)]
pub struct Cont {
    count: i64,
}

#[qspice::main]
fn cont(
    qspice: &mut QSpice,
    cont: &mut Cont,
    _t: f64,
    _data: (
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
    ),
) {
    cont.count += 1;

    if cont.count < 3 {
        let _ = writeln!(qspice, "test message");
    }
}

#[qspice::max]
fn max_ext_step_size(_qspice: &mut QSpice, _inst: &mut Cont, _t: f64) -> f64 {
    1e308 // implement a good choice of max timestep size that depends on struct sCONT
}

#[qspice::trunc]
fn trunc(
    _qspice: &mut QSpice,
    _cont: &mut Cont,
    _t: f64,
    _data: (
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
    ),
    _timestep: &mut f64,
) {
}
