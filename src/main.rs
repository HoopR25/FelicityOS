#![no_std]
#![no_main]

#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use core::fmt;
use core::fmt::Write;


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()])
{
    println!("Running {} tests.", tests.len());
    for test in tests
    {
        test();
    }
}





mod vga_buffer;


#[panic_handler]
fn panic (info: &PanicInfo) -> ! {
    println!("{}", info);
    loop { }
}


#[no_mangle]
pub extern "C" fn _start () -> ! {
    println!("Hello World!");

    #[cfg(test)]
    test_main();
    
    loop { }
}


#[test_case]
fn trivial_assertion()
{
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

