# QSPICE for Rust

Build DLLs to simulate digital functionality for qspice using rust. No unsafe required for i/o to the simulator or to
writing to the simulator console. A few notes:

1. Use this configuration to build DLLs. `march=native` will allow the DLL to use the best instructions for your cpu
   (though it may not run on other machines) and qspice seems to require 32-bit DLLs.

```toml
[build]
rustflags = ["-C", "target-cpu=native"]
target = "i686-pc-windows-msvc"
```

2. Although only safe rust is required in your code, the macro does not generate secure code. The I/O argument number
   and types must be correct, and the macro may not check every case. It it meant as a convenience for non-malicious users.

3. The `Cont` struct can be any type that can be boxed, but it must derive `Default` and functions must use the same type.

4. All functions with qspice macros must be defined in the same module.

## Example

```rust
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
        &mut bool,
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
        &mut bool,
        &mut f32,
    ),
    _timestep: &mut f64,
) {
}
```