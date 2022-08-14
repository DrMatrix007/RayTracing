use sfml::{
    graphics::{Color, Image, Rect, RenderTarget, RenderWindow, Sprite, Texture},
    window::{ContextSettings, Event, Key, Style, VideoMode},
};

use crate::{
    point::{lerp_points, Point, ToPoint},
    ray::Ray,
    shape::{CollisionResult, Shape},
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
        ((start_x.sin(), start_y.sin(), start_x.cos()).to_point(),(end_x.sin(), end_y.sin(), end_x.cos()).to_point())   
    }
    
}
pub struct Screen {
    window: RenderWindow,
    o: Box<dyn Shape>,
    c: Camera,
}

impl Screen {
    pub fn new<T: 'static + Shape>(s: T, c: Camera) -> Self {
        Screen {
            c,
            o: Box::new(s),
            window: RenderWindow::new(
                VideoMode::new(500, 500, 32),
                "title",
                Style::RESIZE | Style::CLOSE,
                &ContextSettings::default(),
            ),
        }
    }
    pub fn run(&mut self) {
        let (xs, ys) = (self.window.size().x, self.window.size().y);
        'main: loop {
            // if target.size() != self.window.size() {
            // target = RenderTexture::new(self.window.size().x,self.window.size().y).unwrap();
            // }
            self.window.clear(Color::BLACK);
            let size = self.window.size();
            let mut target = Texture::new().unwrap();
            let mut image = Image::new(size.x, size.y);
            let mut sprite = Sprite::new();

            let (bottom_left,top_right) = self.c.get_corners((size.x,size.y)); 

            for x in 0..xs {
                for y in 0..ys {
                    let d = lerp_points(
                        bottom_left,
                        top_right,
                        (
                            x as f64 / xs as f64,
                            y as f64 / ys as f64,
                            x as f64 / xs as f64,
                        )
                            .to_point(),
                    );
                    if let CollisionResult::PointCollision(_) =
                        self.o.as_ref().ray_cast(Ray::new(self.c.origin, d))
                    {
                        unsafe {
                            image.set_pixel(x, y, Color::RED);
                        }
                    } else {
                        unsafe {
                            image.set_pixel(x, y, Color::rgb(69, 69, 69));
                        }
                    }
                }
            }
            target
                .load_from_image(
                    &image,
                    Rect::new(
                        0,
                        0,
                        self.window.size().x as i32,
                        self.window.size().y as i32,
                    ),
                )
                .unwrap();
            sprite.set_texture(&target, false);

            self.window.draw(&sprite);
            while let Some(event) = self.window.poll_event() {
                match event {
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
        }
    }
}
