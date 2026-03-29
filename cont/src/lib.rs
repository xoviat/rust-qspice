#[derive(Default)]
pub struct Cont {
    // declare the structure here
}

#[qspice::main]
fn cont(
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
        &mut f32,
    ),
) {
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
        &mut f32,
    ),
    _timestep: &mut f64,
) {
}
