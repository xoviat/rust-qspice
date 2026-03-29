use qspice::Console;
use std::io::Write;

#[derive(Default)]
pub struct Cont {
    count: i64,
}

#[qspice::main]
fn cont(
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
    let mut console = Console::new();

    cont.count += 1;

    if cont.count < 3 {
        let _ = writeln!(&mut console, "test message");
    }
}

#[qspice::max]
fn max_ext_step_size(_inst: &mut Cont, _t: f64) -> f64 {
    1e308 // implement a good choice of max timestep size that depends on struct sCONT
}

#[qspice::trunc]
fn trunc(
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
