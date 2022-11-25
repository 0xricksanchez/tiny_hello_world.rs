#![no_std]
#![no_main]
#![feature(lang_items, intrinsics, start)]
use core::arch::asm;

/// # Safety
///
/// This function is safe AF!
#[no_mangle]
pub unsafe extern "C" fn _start() {
    const HELLO: &str = "Hello, world!\n";
    asm!(
        "syscall",
        in("rax") 1,
        in("rdi") 1,
        in("rsi") HELLO.as_ptr(),
        in("rdx") HELLO.len(),
        );
    asm!(
        "syscall",
        in("rdi") 42,
        in("rax") 60,
        options(noreturn)
        )
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() { }
