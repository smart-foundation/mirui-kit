//! Bare-metal shell: the same UI, with no operating system underneath.
//!
//! This target exists to prove the application crate really is `no_std`:
//! it builds for a bare-metal RISC-V target, renders into a software
//! framebuffer and never allocates through the platform allocator.

#![no_std]
#![no_main]

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use mirui::app::App;
use mirui::render::texture::ColorFormat;
use mirui::surface::framebuf::FramebufSurface;
use mirui::types::PhysicalRect;

const WIDTH: u16 = 800;
const HEIGHT: u16 = 480;
const HEAP_SIZE: usize = 512 * 1024;

// --- critical section ------------------------------------------------------
//
// mirui's reactive runtime needs `critical-section`; on a single core a
// spin lock is the whole implementation.

static CORE_LOCK: AtomicBool = AtomicBool::new(false);

struct SingleCore;

critical_section::set_impl!(SingleCore);

unsafe impl critical_section::Impl for SingleCore {
    unsafe fn acquire() -> critical_section::RawRestoreState {
        while CORE_LOCK.swap(true, Ordering::Acquire) {
            core::hint::spin_loop();
        }
    }

    unsafe fn release(_state: critical_section::RawRestoreState) {
        CORE_LOCK.store(false, Ordering::Release);
    }
}

// --- allocator -------------------------------------------------------------

static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
static HEAP_USED: AtomicUsize = AtomicUsize::new(0);

struct BumpAllocator;

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let base = core::ptr::addr_of!(HEAP) as *mut u8;
        let used = HEAP_USED.fetch_add(layout.size(), Ordering::Relaxed);
        let end = used + layout.size();
        if end > HEAP_SIZE {
            return core::ptr::null_mut();
        }
        unsafe { base.add(used) }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator;

// --- panic -----------------------------------------------------------------

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

// --- application -----------------------------------------------------------

/// Where the rendered frame goes. On real hardware this pushes the rectangle
/// to a panel over SPI; here it is a sink that proves the render path runs.
fn present(_bytes: &[u8], _area: PhysicalRect) {}

fn run() -> ! {
    let backend = FramebufSurface::with_format(WIDTH, HEIGHT, ColorFormat::RGB565, present);
    let mut app = App::new(backend);
    app.with_default_widgets().with_default_systems();
    fm::install(&mut app);
    app.render().expect("frame renders");
    loop {
        core::hint::spin_loop();
    }
}

/// Freestanding entry point: a real board would call this from its reset
/// vector instead of from a C runtime.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    run()
}
