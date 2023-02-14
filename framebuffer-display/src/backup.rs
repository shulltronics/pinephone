use std::time::{Duration, Instant};

use memmap;
use linuxfb::Framebuffer;
use linuxfb::double::Buffer;

use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Bgr888;

fn main() {
    println!("Hello from Rust on PinePhone!\n");

    let mut fb = Framebuffer::new("/dev/fb0").unwrap();

    println!("framebuffer size: {:?}\n", fb.get_size());
    
    //println!("Trying double buffer...");

    //let db = Buffer::new(fb).unwrap();

    let mut buffer = fb.map().unwrap();

    println!("starting direct buffer draw...");
    let start: Instant = Instant::now();
    for i in 0..buffer.len() {
        buffer[i] = 255u8;
    }
    let duration: Duration = start.elapsed();
    println!("done drawing. Delta t = {:?}\n", duration);

    println!("Running with BGRA type...");
    let tc: BGRA = BGRA::new(255, 0, 0);
    let start = Instant::now();
    splash_bgra(tc, &mut buffer);
    let dur = start.elapsed();
    println!("duration: {:?}\n", dur);

    println!("Running with BGRA type and splash fill...");
    let tc: BGRA = BGRA::new(255, 255, 0);
    let start = Instant::now();
    splash_fill(tc, &mut buffer);
    let dur = start.elapsed();
    println!("duration: {:?}\n", dur);

    for i in 0..255 {
        let color = wheel(i);

        let start = Instant::now();
        splash_cast(color, &mut buffer);
        let dur   = start.elapsed();
        println!("duration: {:?}", dur);
    }
}

// Write the color to the whole framebuffer
fn splash_chunk(color: Bgr888, fb: &mut memmap::MmapMut) {
    for chunk in fb.chunks_exact_mut(4) {
        chunk[0] = color.b();
        chunk[1] = color.g();
        chunk[2] = color.r();
        chunk[3] = 0;
        //println!("{:?}", chunk);
    }
}

fn splash_cast(color: Bgr888, fb: &mut memmap::MmapMut) {
    let (_prefix, pixels, _suffix) = unsafe { fb.align_to_mut::<u32>() };
    for i in 0..pixels.len() {
        pixels[i] = color.into_storage() << 8 | 0x00000000u32;
    }
}

fn splash_bgra(color: BGRA, fb: &mut memmap::MmapMut) {
    let (_prefix, pixels, _suffix) = unsafe { fb.align_to_mut::<u32>() };
    for i in 0..pixels.len() {
        pixels[i] = color.into_storage();
    }
}

fn splash_fill(color: BGRA, fb: &mut memmap::MmapMut) {
    let (_prefix, pixels, _suffix) = unsafe { fb.align_to_mut::<u32>() };
    pixels.fill(color.into_storage());
}

fn wheel(mut w: u8) -> Bgr888 {
    if w < 85 {
        return Bgr888::new(255u8 - w*3, w*3, 0);
    } else if w < 170 {
        w -= 85;
        return Bgr888::new(0, 255u8 - w*3, w*3);
    } else {
        w -= 170;
        return Bgr888::new(w*3, 0, 255u8 - w*3);
    }
}


struct BGRA {
    raw: u32
}

impl BGRA {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            raw: (b as u32) << 24 | (g as u32) << 16 | (r as u32) << 8,
        }
    }

    fn into_storage(&self) -> u32 {
        return self.raw;
    }
}
