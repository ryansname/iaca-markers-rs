#![feature(asm)]
//! Provides macros to bench your application with [iaca](https://software.intel.com/en-us/articles/intel-architecture-code-analyzer)

#[macro_use]

/// This macro signifies the start block of the section to be analysed.
/// Put it after the loop initialization
#[macro_export]
macro_rules! iaca_start {
    () => {
        unsafe {
            asm!("mov ebx, 111" :::: "intel");
            asm!(".byte 0x64, 0x67, 0x90" :::: "intel");
        }
    }
}

/// Signal the end of the block to be analyzed.
/// Put it at the end of the block to be analyzed.
#[macro_export]
macro_rules! iaca_end {
    () => {
        unsafe {
            asm!("mov ebx, 222" :::: "intel");
            asm!(".byte 0x64, 0x67, 0x90" :::: "intel");
        }
    }
}

#[test]
fn it_wont_work() {
    iaca_start!();

    let mut a = 4;
    a += 10;
    
    iaca_end!();

    assert_eq!(a, 14);
}
