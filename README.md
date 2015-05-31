# iaca-markers-rs
Adds [IACA](https://software.intel.com/en-us/articles/intel-architecture-code-analyzer) compatible macros for rust to identify and fix bottlenecks and improve throughput.

## Usage

When compiling with IACA enabled your executable will not run.

1. Instead add the macros around the section you wish to profile as below.
2. Compile in release mode: `cargo build --release`.
3. Run IACA from the link above (eg.): `./iaca.sh -64 ~/path/to/exe/release/librustlib.rlib`
4. Enjoy the extremely detailed output.


```rust
#![feature(asm)]

#[macro_use] extern crate iaca_markers;

fn main() {
    let mut a = 0;
    for i in 0..100 {
        iaca_start!();

        // Do some work
        a += i;
    }
    iaca_end!();

    println!("Summed to {}", a);
}
```

## Notes
I have run this successfully with `rustc 1.2.0-nightly (e4c64a149 2015-05-27)`. It may require beta or nightly rust for the moment until the asm feature is stabilized.
