#![allow(dead_code, unused_variables)]
use raylib::prelude::*;
use std::time::SystemTime;
trait ProcessObject {
    fn process(&mut self, time: f32, w: u32, h: u32);
    fn draw(&self, draw_handler: &mut RaylibDrawHandle);
    fn detect_collision(&mut self, w: u32, h: u32) -> bool;
}

impl ProcessObject for Ball {
    fn process(&mut self, time: f32, w: u32, h: u32) {
        println!(
            "speed: {},{} current: {},{} acel: {}",
            self.speed.x, self.speed.y, self.current.x, self.current.y, self.acel.y
        );
        self.speed = self.speed + self.acel * time;
        self.acel.x = self.acel.x / 1.15;
        let delta_y = self.speed.y * time + self.acel.y * time.powf(2.0) / 2.0;
        self.current.y = self.current.y + delta_y;
        self.current.x = self.current.x + self.speed.x * time;

        if self.detect_collision(w, h) {
            self.speed = -self.speed * 0.9;
        }
        // if self.detect_collision(w, h) {
        //     self.y = h as f64 - self.radius;
        //     self.speed = -self.speed;
        //     self.y -= delta_y;
        // }
    }
    fn detect_collision(&mut self, w: u32, h: u32) -> bool {
        let tmp = self.current + self.radius;
        if tmp.x >= w as f32 && tmp.y > 0.0 && tmp.y < h as f32 {
            self.current.x = w as f32 - self.radius;
            self.speed.y = -self.speed.y;
            return true;
        }
        if tmp.x < 0.0 && tmp.y > 0.0 && tmp.y < h as f32 {
            self.current.x = self.radius;
            self.speed.y = -self.speed.y;
            return true;
        }
        if tmp.y >= h as f32 && tmp.x > 0.0 && tmp.x < w as f32 {
            self.current.y = h as f32 - self.radius;
            self.speed.x = -self.speed.x;
            return true;
        }
        if tmp.y <= 0.0 && tmp.x > 0.0 && tmp.x < w as f32 {
            self.current.y = self.radius;
            self.speed.x = -self.speed.x;
            return true;
        }
        false
    }
    fn draw(&self, draw_handler: &mut RaylibDrawHandle) {
        draw_handler.draw_circle(
            self.current.x as i32,
            self.current.y as i32,
            self.radius as f32,
            self.color,
        );
    }
}

struct Ball {
    init: Vector2,
    current: Vector2,
    acel: Vector2,
    speed: Vector2,
    radius: f32,
    color: Color,
}
struct BallBuilder {
    init: Vector2,
    current: Vector2,
    acel: Vector2,
    speed: Vector2,
    radius: f32,
    color: Color,
}
impl BallBuilder {
    fn new() -> BallBuilder {
        BallBuilder {
            init: Vector2 { x: 0.0, y: 0.0 },
            current: Vector2 { x: 0.0, y: 0.0 },
            acel: Vector2 { x: 0.0, y: 0.0 },
            speed: Vector2 { x: 0.0, y: 0.0 },
            radius: 0.0,
            color: Color::RED,
        }
    }
    fn pos(&mut self, x: f32, y: f32) -> &mut BallBuilder {
        self.init.x = x;
        self.init.y = y;
        self.current = self.init;
        self
    }
    fn acel(&mut self, x: f32, y: f32) -> &mut BallBuilder {
        self.acel.x = x;
        self.acel.y = y;
        self
    }
    fn speed(&mut self, x: f32, y: f32) -> &mut BallBuilder {
        self.speed.x = x;
        self.speed.y = y;
        self
    }
    fn radius(&mut self, radius: f32) -> &mut BallBuilder {
        self.radius = radius;
        self
    }
    fn color(&mut self, color: Color) -> &mut BallBuilder {
        self.color = color;
        self
    }
    fn build(&self) -> Ball {
        Ball {
            init: self.init,
            current: self.current,
            acel: self.acel,
            speed: self.speed,
            radius: self.radius,
            color: self.color,
        }
    }
}

struct Model {
    width: u32,
    height: u32,
    last_time: f64,
    period: f64,
    objects: Vec<Box<dyn ProcessObject>>,
}

impl Model {
    pub fn process(&mut self) {
        let curr_time = get_time_s();
        let time_delta = curr_time - self.last_time;
        //println!("time: {}", time_delta);
        if time_delta >= self.period {
            self.last_time = curr_time;
            for object in self.objects.iter_mut() {
                object.process(self.period as f32, self.width, self.height);
            }
        }
    }
    pub fn draw(&mut self, draw_handler: &mut RaylibDrawHandle) {
        for object in self.objects.iter_mut() {
            object.draw(draw_handler);
        }
    }
}

fn get_time_s() -> f64 {
    let duration_since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    duration_since_epoch.as_millis() as f64 / 1000.0
}
fn main() {
    const G: f32 = 1000.0;
    const WINDOW_WIDTH: u32 = 640;
    const WINDOW_HEIGHT: u32 = 480;
    const MODEL_PERIOD: f64 = 0.01;
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .title("Bouncing Balls")
        .build();
    let ball1 = BallBuilder::new()
        .pos(20 as f32, (WINDOW_HEIGHT / 2) as f32)
        .radius(20.0)
        .acel(9000.0, G)
        .build();

    let ball2 = BallBuilder::new()
        .pos(WINDOW_WIDTH as f32 - 50.0, (WINDOW_HEIGHT / 2) as f32)
        .radius(40.0)
        .acel(20000.0, G)
        .color(Color::BLUE)
        .build();

    let mut model = Model {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        period: MODEL_PERIOD,
        last_time: get_time_s(),
        objects: vec![Box::new(ball1), Box::new(ball2)],
    };
    let mouse_vec = rl.get_mouse_position();
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        model.process();
        model.draw(&mut d);
    }
}
