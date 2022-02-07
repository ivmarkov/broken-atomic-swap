use core::sync::atomic::{AtomicBool, AtomicU16, AtomicU32, Ordering};

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    println!("About to do some atomic checks");

    // OK: u32 (32 bit)
    {
        let atomic = AtomicU32::new(0);

        assert_eq!(atomic.load(Ordering::SeqCst), 0);

        // Loading `1` in the atomic, swap should return the previous value (`0`)
        assert_eq!(atomic.swap(1, Ordering::SeqCst), 0);

        // This works
        assert_eq!(atomic.load(Ordering::SeqCst), 1);

        // This works too
        assert_eq!(atomic.swap(1, Ordering::SeqCst), 1);
    }

    // BROKEN: u16 (16 bit)
    {
        let atomic = AtomicU16::new(0);

        assert_eq!(atomic.load(Ordering::SeqCst), 0);

        // Loading `1` in the atomic, swap should return the previous value (`0`)
        assert_eq!(atomic.swap(1, Ordering::SeqCst), 0);

        // This works
        assert_eq!(atomic.load(Ordering::SeqCst), 1);

        // This breaks?!
        assert_eq!(atomic.swap(1, Ordering::SeqCst), 1);
    }

    // BROKEN: bool (8 bit)
    {
        let atomic = AtomicBool::new(false);

        assert_eq!(atomic.load(Ordering::SeqCst), false);

        // Loading `true` in the atomic, swap should return the previous value (`false`)
        assert_eq!(atomic.swap(true, Ordering::SeqCst), false);

        // This works
        assert_eq!(atomic.load(Ordering::SeqCst), true);

        // This breaks?!
        assert_eq!(atomic.swap(true, Ordering::SeqCst), true);
    }

    println!("All done correctly!")
}
