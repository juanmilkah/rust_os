#![no_std] /*disable rust standard library*/
#![no_main] /* disable rust entry points*/

use core::panic::PanicInfo;

/*entry pont of the program*/
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

/* function called on panic*/
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
