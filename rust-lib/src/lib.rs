#![no_std]

#[cfg(not(test))]
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}

#[no_mangle]
pub extern "C" fn average(one: usize, two: usize, three: usize) -> usize {
    (one + two + three) / 3
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        std::assert_eq!(result, 4);
    }

    #[test]
    fn computes_average() {
        let result = average(3, 6, 9);
        std::assert_eq!(result, 6);
    }
}
