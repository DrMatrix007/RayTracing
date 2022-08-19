use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, RwLock,
    },
    thread::{self},
    time::Instant,
};

use sfml::{
    graphics::{Color, Image, Rect, RenderTarget, RenderWindow, Sprite, Texture, View},
    system::Vector2f,
    window::{ContextSettings, Event, Key, Style, VideoMode},
};

use crate::{
    point::{lerp_points, Point, ToPoint},
    ray::Ray,
    shape::Shape,
    world::World,
};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub origin: Point,
    pub rot_x: f64,
    pub rot_y: f64,
    pub fov: f64,
}
impl Camera {
    fn new(origin: Point, rot_x: f64, rot_y: f64, fov: f64) -> Self {
        Self {
            origin,
            rot_x,
            rot_y,
            fov,
        }
    }
    fn bottom_left(&self, window_size: (impl Into<f64>, impl Into<f64>)) -> Point {
        let end_x = -(self.fov / 2.0) + self.rot_x;
        let end_y = -(self.fov / 2.0) * window_size.1.into() / window_size.0.into() + self.rot_y;

        (end_x.sin(), end_y.sin(), end_x.cos()).to_point()
    }
    fn top_right(&self, window_size: (impl Into<f64>, impl Into<f64>)) -> Point {
        let start_x = (self.fov / 2.0) + self.rot_x;
        let start_y = (self.fov / 2.0) * window_size.1.into() / window_size.0.into() + self.rot_y;

        (start_x.sin(), start_y.sin(), start_x.cos()).to_point()
    }
    fn get_corners(&self, window_size: (impl Into<f64>, impl Into<f64>)) -> (Point, Point) {
        let r: f64 = window_size.1.into() / window_size.0.into();
        let start_x = (self.fov / 2.0) + self.rot_x;
        let start_y = (self.fov / 2.0) * r + self.rot_y;
        let end_x = -(self.fov / 2.0) + self.rot_x;
        let end_y = -(self.fov / 2.0) * r + self.rot_y;
        (
            (start_x.sin(), start_y.sin(), start_x.cos()).to_point(),
            (end_x.sin(), end_y.sin(), end_x.cos()).to_point(),
        )
    }
}

struct ImageWrapper(Image);

unsafe impl Send for ImageWrapper {}
unsafe impl Sync for ImageWrapper {}

pub struct Screen {
    window: RenderWindow,
    world: World,
    c: Camera,
}
unsafe impl Send for Screen {}
unsafe impl Sync for Screen {}
impl Screen {
    pub fn new<T: 'static + Shape>(s: T, c: Camera) -> Self {
        Screen {
            c,
            world: World {
                object: Arc::new(s),
            },
            window: RenderWindow::new(
                VideoMode::new(500, 500, 32),
                "title",
                Style::RESIZE | Style::CLOSE,
                &ContextSettings::default(),
            ),
        }
    }

    pub fn run(mut self) {
        let (mut xs, mut ys) = (self.window.size().x, self.window.size().y);
        let thread_counter = Arc::new(AtomicUsize::new(0));

        'main: loop {
            let start = Instant::now();
            // let mut pool = Vec::<JoinHandle<()>>::new();
            // if target.size() != self.window.size() {
            // target = RenderTexture::new(self.window.size().x,self.window.size().y).unwrap();
            // }            let a = self.window.default_view().to_owned();
            self.window.set_view(&View::new(
                Vector2f::new(xs as f32 / 2.0, ys as f32 / 2.0),
                Vector2f::new(xs as f32, ys as f32),
            ));
            self.window.clear(Color::BLACK);
            let mut target = Texture::new().unwrap();
            let image = Image::new(xs, ys);
            let mut sprite = Sprite::new();
            let image = Arc::new(RwLock::new(ImageWrapper(image)));

            let (bottom_left, top_right) = self.c.get_corners((xs, ys));
            {
                let world = Arc::new(self.world.clone());
                for x in 0..xs {
                    let thread_counter = thread_counter.clone();
                    let world = world.clone();
                    let image = image.clone();
                    let x_div = x as f64 / xs as f64;
                    thread_counter.fetch_add(1, Ordering::Relaxed);

                    thread::spawn(move || {
                        let mut data = Vec::with_capacity(ys as usize);
                        let mut r = Ray::new(self.c.origin, Point::ZERO);
                        for y in 0..ys {
                            r.direction = lerp_points(
                                bottom_left,
                                top_right,
                                (x_div, y as f64 / ys as f64, x_div).to_point(),
                            );
                            data.push(world.as_ref().get_color(r));
                        }
                        match image.write() {
                            Ok(mut i) => unsafe {
                                for y in 0..ys {
                                    i.0.set_pixel(x, y, *(data.get_unchecked(y as usize)));
                                }
                            },
                            _ => {}
                        }
                        thread_counter.fetch_sub(1, Ordering::Relaxed);
                    });
                }
            }
            while thread_counter.load(Ordering::Relaxed) > 0 {}
            let image = &image.try_read().unwrap().0;
            target
                .load_from_image(
                    image,
                    Rect::new(
                        0,
                        0,
                        self.window.size().x as i32,
                        self.window.size().y as i32,
                    ),
                )
                .unwrap();
            // sprite.set_origin(Vector2f::new(xs as f32 / 2.0, ys as f32));
            sprite.set_texture(&target, false);
            self.window.draw(&sprite);
            while let Some(event) = self.window.poll_event() {
                match event {
                    Event::Resized { width, height } => {
                        (xs, ys) = (width, height);
                    }
                    Event::Closed => {
                        break 'main;
                    }



                
                    Event::KeyPressed { code, .. } => match code {
                        Key::D => {
                            self.c.rot_x += 0.1;
                        }
                        Key::A => {
                            self.c.rot_x -= 0.1;
                        }
                        Key::W => {
                            self.c.rot_y += 0.1;
                        }
                        Key::S => {
                            self.c.rot_y -= 0.1;
                        }

                        Key::Right => {
                            self.c.origin.x += 0.1;
                        }
                        Key::Left => {
                            self.c.origin.x -= 0.1;
                        }
                        Key::Up => {
                            self.c.origin.z += 0.1;
                        }
                        Key::Down => {
                            self.c.origin.z -= 0.1;
                        }

                        _ => {}
                    },
                    _ => {}
                }
            }

            self.window.display();
            // break;
            println!("fps: {}", 1.0 / (Instant::now() - start).as_secs_f64());
        }
    }
}
