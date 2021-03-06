#![no_std]
#![feature(
    asm,
    panic_info_message,
    lang_items,
    allocator_api,
    alloc_error_handler,
    alloc_prelude,
    untagged_unions,
    core_intrinsics,
)]

#[macro_use]
extern crate alloc;

#[macro_export]
macro_rules! print {
    ($($args:tt)+) => ({
        use core::fmt::Write;
        let _ = write!(uart::Uart::new(), $($args)+);
    });
}

#[macro_export]
macro_rules! println
{
	() => ({
		   print!("\r\n")
		   });
	($fmt:expr) => ({
			print!(concat!($fmt, "\r\n"))
			});
	($fmt:expr, $($args:tt)+) => ({
			print!(concat!($fmt, "\r\n"), $($args)+)
			});
}

#[lang = "eh_personality"] extern fn eh_personality() {}

#[panic_handler]
fn panic(_info :& PanicInfo) -> ! {
    println!("Panic! ");
    if let Some(p) = _info.location() {
		println!(
				"line {}, file {}: {}",
				p.line(),
				p.file(),
				_info.message().unwrap()
				);
	}
    else {
        println!("no information");
    }
    abort();
}

#[no_mangle]
extern "C" fn abort() -> !{
    loop{
        unsafe{
            asm!("wfi"::::"volatile");
        }
    }
}

#[no_mangle]
extern "C" fn kernel_init(){
    Uart::new().init();
    trap::init(0);
    memory::init();
    memory::test();
    console::init();
    plic::init();
    task::init();

    process::start_init_process();
}

#[no_mangle]
extern "C" fn kernel_start(hartid : usize){
    trap::init(hartid);
}

mod uart;
mod sync;
mod plic;
mod cpu;
mod memory;
mod task;
mod libs;
mod interrupt;
mod interact;
mod virtio;
use interact::console;
use interrupt::trap;
use task::process;
use uart::Uart;
// use alloc::{prelude::v1::*};
use core::{panic::PanicInfo};
