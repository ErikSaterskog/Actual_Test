use eframe::{egui, epi};
//use chrono::Duration;
use ::egui::Context;
use crate::egui::TextureId;
//use ::egui::mutex::Mutex;
use rand::Rng;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::SystemTime;
use std::{
    sync::mpsc::channel,
    time::Duration,
    time::Instant,
};
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::spawn;

mod vec;
use crate::vec::Vec3;



//#[derive(Default)]
#[derive(Debug)]
struct MyApp {
    pixels: std::vec::Vec<u8>,
    texture: Option<((usize, usize), egui::TextureId)>,
    gamma: i32,
    fps: f32,
    frame_time: u128,
    now: std::time::Instant,
    calculating: bool,
    cont_updates: bool,
    updates: u32,
    tx: std::sync::mpsc::Sender<(usize, usize, Vec3)>,
    rx: std::sync::mpsc::Receiver<(usize, usize, Vec3)>,
}



impl Default for MyApp {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let now = Instant::now();
        MyApp {
            pixels: vec![0],
            texture: std::option::Option::Some(((0 as usize,0 as usize),TextureId::User(0))),
            gamma: 100,
            fps: 0.0,
            frame_time: 0,
            now,
            calculating: false,
            cont_updates: false,
            updates: 0,
            tx,
            rx,
        }
    }
}

fn calculation(
    x: usize,
    y: usize,
    size: (usize, usize),
) -> (usize, usize, Vec3) {
    
    return (x, y, Vec3{x:x as f32/(size.0 as f32)*255.0, y:y as f32/(size.1 as f32)*255.0, z:0.0});
}


impl epi::App for MyApp {
    fn name(&self) -> &str {
        "Gui_Test"
    }

    fn setup(
            &mut self,
            _ctx: &egui::CtxRef,
            frame: &mut epi::Frame<'_>,
            _storage: Option<&dyn epi::Storage>,
        ) {
        
        let image_data = include_bytes!("test_large.png");
        let image = image::load_from_memory(image_data).expect("Failed to load image");
        let image_buffer = image.to_rgba8();
        let size = (image.width() as usize, image.height() as usize);
        self.pixels = image_buffer.into_vec();
        assert_eq!(size.0 * size.1 * 4, self.pixels.len());
        let pixels: Vec<_> = self.pixels
            .chunks_exact(4)
            .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
            .collect();
        // Allocate a texture:
        let texture = frame
            .tex_allocator()
            .alloc_srgba_premultiplied(size, &pixels);
        
        self.texture = Some((size, texture));
        self.gamma = 100;
        self.fps = 0.0;
        let now = Instant::now();
        self.frame_time = now.elapsed().as_micros();
        self.updates = 0;
        self.cont_updates = true;
        let (tx, rx) = std::sync::mpsc::channel();
        self.rx = rx;

        
        thread::spawn(move || {
            loop{
                
                thread::sleep(Duration::from_nanos(1));
                // for x in 0..size.0 {
                //     for y in 0..size.1 {
                //         tx.send(calculation(x, y, size));
                //     }
                // }
                for _i in 0..100 {
                    let mut rng = rand::thread_rng();
                    let x = rng.gen_range(0..size.0);
                    let y = rng.gen_range(0..size.1);
                    let result = calculation(x, y, size);
                    tx.send(result).unwrap();
                }
            }
        });
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        self.updates += 1;
        println!("{:?}", self.fps);
        self.frame_time = self.now.elapsed().as_micros();
        self.fps = 1000000.0 / (self.frame_time as f32);
        self.now = Instant::now();

        egui::CentralPanel::default().show(ctx, |ui| {
                let Some((size, texture)) = self.texture else {panic!()};
                //ui.heading("This is an image:");
                ui.style_mut().spacing.slider_width = size.0 as f32;
                ui.image(texture, egui::Vec2::new(size.0 as f32, size.1 as f32));
                ui.add(egui::Slider::new(&mut self.gamma, 0..=200).text("Gamma Correction"));
                
                loop {
                    match self.rx.try_recv() {
                        Ok(result) => {
                            let result: (usize, usize, Vec3) = result;
                            let x = result.0;
                            let y = result.1;
                            let color = result.2;
                            self.pixels[(4*x+y*500*4+0) as usize] = color.x as u8;
                            self.pixels[(4*x+y*500*4+1) as usize] = color.y as u8;
                            self.pixels[(4*x+y*500*4+2) as usize] = color.z as u8;
                            self.pixels[(4*x+y*500*4+3) as usize] = 255;
                        }
                        Err(_) => {
                            //println!("{:?}", "Error!");
                            break;
                        }
                    }
                }        
                
                
                let mut pixels_gamma = self.pixels.clone();
                for i in 0..pixels_gamma.len() {
                    pixels_gamma[i] = ((self.pixels[i] as f32).powf((self.gamma as f32) / 100.0)) as u8;
                }

                
                if ui.button("Save").clicked() {
                    println!("{:?}","To be implemented...")
                }
                
                ui.checkbox(&mut self.cont_updates, "Continuous Updates");
                
                if self.cont_updates {
                    ctx.request_repaint();

                    let pixels: Vec<_> = pixels_gamma
                        .chunks_exact(4)
                        .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
                        .collect();

                    // Allocate a texture:
                    let texture = frame
                        .tex_allocator()
                        .alloc_srgba_premultiplied((size.0, size.1), &pixels);
                    //let size = egui::Vec2::new(size.0 as f32, size.1 as f32);
                    self.texture = Some((size, texture));
                }
                if ui.button("Update Image").clicked() {
                    let pixels: Vec<_> = pixels_gamma
                        .chunks_exact(4)
                        .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
                        .collect();

                    // Allocate a texture:
                    let texture = frame
                        .tex_allocator()
                        .alloc_srgba_premultiplied((size.0, size.1), &pixels);
                    //let size = egui::Vec2::new(size.0 as f32, size.1 as f32);
                    self.texture = Some((size, texture));
                }
                
        // Resize the native window to be just the size we need it to be:
        //frame.set_window_size(ctx.used_size());
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(MyApp::default()), options);
}