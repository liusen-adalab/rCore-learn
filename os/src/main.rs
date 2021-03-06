#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(const_in_array_repeat_expressions)]

#![feature(alloc_error_handler)]
extern crate alloc;

#[macro_use]
extern crate bitflags;

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod syscall;
mod trap;
mod loader;
mod config;
mod task;
mod timer;
mod mm;
mod fs;
mod drivers;



global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

/*******************************************************************
_start program entry
******************************************************************/
#[no_mangle]
pub fn rust_main() {
    clear_bss();
    println!("[kernel] Hello, world!");

    mm::init();

    task::add_initproc();
    println!("after add initproc");

    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();

    loader::list_apps();

    task::run_tasks();
    panic!("Unreachable in rust_main!");
}

