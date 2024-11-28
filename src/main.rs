#![allow(dead_code, unused_variables)]
use raylib::prelude::*;
use std::time::SystemTime;
trait ProcessObject {
    fn process(&mut self, time: f64, w: u32, h: u32);
    fn draw(&self, draw_handler: &mut RaylibDrawHandle);
    fn detect_collision(&self, w: u32, h: u32) -> bool;
}

impl ProcessObject for Ball {
    fn process(&mut self, time: f64, w: u32, h: u32) {
        self.speed = self.speed + self.acel * time;
        let delta_y = self.speed * time + self.acel * time.powf(2.0) / 2.0;
        self.y = self.y + delta_y;
        if self.detect_collision(w, h) {
            self.y = h as f64 - self.radius;
            self.speed = -self.speed;
            self.y -= delta_y;
        }
    }
    fn detect_collision(&self, w: u32, h: u32) -> bool {
        if self.y + self.radius < h.into() {
            false
        } else {
            true
        }
    }
    fn draw(&self, draw_handler: &mut RaylibDrawHandle) {
        draw_handler.draw_circle(self.x as i32, self.y as i32, self.radius as f32, self.color);
    }
}

struct Ball {
    init_x: u32,
    init_y: u32,
    x: f32,
    y: f64,
    radius: f64,
    speed: f64,
    acel: f64,
    color: Color,
    total: f64,
}

impl Ball {
    pub fn new(x: u32, y: u32, radius: f64, acel: f64, color: Color) -> Self {
        Self {
            init_x: x,
            init_y: y,
            x: x as f32,
            y: y as f64,
            acel: acel,
            radius: radius,
            speed: 0.0,
            color: color,
            total: 0.0,
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
        if time_delta >= self.period {
            self.last_time = curr_time;
            for object in self.objects.iter_mut() {
                object.process(self.period, self.width, self.height);
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
    const G: f64 = 100000.0;
    const WINDOW_WIDTH: u32 = 640;
    const WINDOW_HEIGHT: u32 = 480;
    const MODEL_PERIOD: f64 = 0.01;
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .title("Bouncing Balls")
        .build();
    let ball1 = Ball::new(WINDOW_WIDTH / 2, WINDOW_HEIGHT / 2, 20.0, G, Color::RED);
    let ball2 = Ball::new(
        WINDOW_WIDTH / 2 + 100,
        WINDOW_HEIGHT / 2,
        20.0,
        G / 2.0,
        Color::BLUE,
    );
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
