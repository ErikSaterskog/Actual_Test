use eframe::{egui, epi};
use chrono::Duration;
use ::egui::Context;
use crate::egui::TextureId;
//use ::egui::mutex::Mutex;
use rand::Rng;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
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
    var2: f32,
    calculating: bool,
    tx: std::sync::mpsc::Sender<(usize, usize, Vec3)>,
    rx: std::sync::mpsc::Receiver<(usize, usize, Vec3)>,
}



impl Default for MyApp {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        MyApp {
            pixels: vec![0],
            texture: std::option::Option::Some(((0 as usize,0 as usize),TextureId::User(0))),
            gamma: 100,
            var2: 0.0,
            calculating: false,
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
    let duration = Duration::seconds(2)
        .to_std()
        .expect("What is this text?");
    thread::sleep(duration);
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
        self.gamma = 1;
        self.var2 = 2.0;
        
        (self.tx, self.rx) = mpsc::channel();
        
        thread::spawn(move || {
            loop {
                match self.rx.try_recv() {
                    Ok(result) => {
                        //let result: (usize, usize, Vec3) = result;
                        let x = result.0;
                        let y = result.1;
                        let color = result.2;
                        self.pixels[(4*x+y*500*4+0) as usize] = color.x as u8;
                        self.pixels[(4*x+y*500*4+1) as usize] = color.y as u8;
                        self.pixels[(4*x+y*500*4+2) as usize] = color.z as u8;
                        self.pixels[(4*x+y*500*4+3) as usize] = 255;
                    }
                    Err(_) => {
                        println!("{:?}", "Error!");
                        //break;
                    }
                }
            }        
        });
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        
        //println!("{:?}",frame_nr());
        //ctx.request_repaint();
        
        
        egui::CentralPanel::default().show(ctx, |ui| {
                let Some((size, texture)) = self.texture else {panic!()};
                //ui.heading("This is an image:");
                ui.style_mut().spacing.slider_width = size.0 as f32;
                ui.image(texture, egui::Vec2::new(size.0 as f32, size.1 as f32));
                ui.add(egui::Slider::new(&mut self.gamma, 0..=200).text("Gamma Correction"));

                


                if ui.button("Update image").clicked() {
                    //println!("{:?}", self.pixels);
                    //update image
                    //self.pixels[0]=self.gamma as u8;
                    //for id in 0..self.pixels.len() {
                    //    self.pixels[id] = self.gamma as u8;
                    //}
                    let mut pixels_gamma = self.pixels.clone();
                    for i in 0..pixels_gamma.len() {
                        pixels_gamma[i] = ((self.pixels[i] as f32).powf((self.gamma as f32) / 100.0)) as u8;
                    }

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
                if ui.button("Send").clicked() {
                    println!("{:?}","To be implemented...")
                }
                if ui.button("Recieve").clicked() {
                    println!("{:?}","To be implemented...")
                }
                if ui.button("Save").clicked() {
                    println!("{:?}","To be implemented...")
                }
                if ui.button("Start").clicked() {
                    if self.calculating == false {
                        self.calculating = true;
                        println!("{:?}","Started calculation..");
                        
                        
                        let mut rng = rand::thread_rng();
                        let x = rng.gen_range(0..size.0);
                        let y = rng.gen_range(0..size.1);

                        //thread::spawn(move || {
                        //    tx.send(calculation(x, y, size)).unwrap();
                        //});

                        //let computation = thread::spawn(move|| {
                        //    // Some expensive computation.
                        //    return calculation(x, y, size)
                        //});

                        //let result = computation.unwrap();//.join().unwrap();
                        //println!("{:?}",result);
                        
                        
                        //let mut thread_test = My::new(Vec3{x:0.0, y:0.0, z:0.0});
	                    //thread_test.start();

                        println!("{:?}","Finished calculation");
                        // for i in 0..10 {
                        //     let tx = tx.clone();
                        //     thread::spawn(move|| {
                        //         let duration = Duration::seconds(i)
                        //             .to_std()
                        //             .expect("What is this text?");
                        //         thread::sleep(duration);
                        //         println!("{:?}","Finished calculation");
                        //         tx.send(i).unwrap();
                        //     });
                        // }

                        // let computation = thread::spawn(|| {
                        //     // Some expensive computation.
                        //     let mut rng = rand::thread_rng();
                        
                        //     let color = calculation(x, y);
                        //     let duration = Duration::seconds(3)
                        //         .to_std()
                        //         .expect("What is this text?");
                        //     thread::sleep(duration);
                        //     println!("{:?}", color);
                        //     
                        //     println!("{:?}","Finished calculation");
                        // });
                        self.calculating = false
                    } else {
                        println!("{:?}","Already calculating...")
                    }
                    
                }
                //self.pixels[0] = 

                if ui.button("Pause").clicked() {
                    println!("{:?}","To be implemented...");


                //println!("{:?}", self.gamma)
                //ui.heading("This is an image you can click:");
                //ui.add(egui::ImageButton::new(texture, size));
            //}
        }
        //let Self { name, age } = self;

        // egui::CentralPanel::default().show(ctx, |ui| {
        //     ui.heading("My egui Application");
        //     ui.horizontal(|ui| {
        //         ui.label("Your name: ");
        //         ui.text_edit_singleline(name);
        //     });
        //     ui.add(egui::Slider::new(age, 0..=120).text("age"));
        //     if ui.button("Click each year").clicked() {
        //         *age += 1;
        //     }
        //     ui.label(format!("Hello '{}', age {}", name, age));
        // });

        // Resize the native window to be just the size we need it to be:
        //frame.set_window_size(ctx.used_size());
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(MyApp::default()), options);
}