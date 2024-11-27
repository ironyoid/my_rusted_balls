#![allow(dead_code, unused_variables)]
use raylib::prelude::*;
const G: f32 = 9.82;
trait ProcessObject {
    fn process(&mut self, time: f32, w: u32, h: u32);
    fn draw(&self, draw_handler: &mut RaylibDrawHandle);
}

impl ProcessObject for Ball {
    fn process(&mut self, time: f32, w: u32, h: u32) {
        if self.y < h - self.radius {
            self.speed += self.acel * time;
            self.delta_y += self.speed * time + self.speed * time.powf(2.0) / 2.0;
            self.y = self.delta_y.round() as u32;
        } else {
            self.y = h - self.radius;
        }
    }
    fn draw(&self, draw_handler: &mut RaylibDrawHandle) {
        draw_handler.draw_circle(self.x as i32, self.y as i32, self.radius as f32, self.color);
    }
}

impl ProcessObject for Square {
    fn process(&mut self, time: f32, w: u32, h: u32) {
        print!("Square y={}\n", self.y);
    }
    fn draw(&self, draw_handler: &mut RaylibDrawHandle) {}
}

struct Ball {
    init_x: u32,
    init_y: u32,
    x: u32,
    y: u32,
    delta_y: f32,
    radius: u32,
    speed: f32,
    acel: f32,
    color: Color,
    name: String,
}

impl Ball {
    pub fn new(x: u32, y: u32, radius: u32, acel: f32, name: String) -> Self {
        Self {
            init_x: x,
            init_y: y,
            x: x,
            y: y,
            acel: acel,
            delta_y: y as f32,
            radius: radius,
            speed: 0.0,
            color: Color::RED,
            name: name,
        }
    }
}

struct Square {
    init_x: u32,
    init_y: u32,
    w: u32,
    h: u32,
    x: u32,
    y: u32,
    speed: u32,
    color: Color,
    name: String,
}

impl Square {
    pub fn new(x: u32, y: u32, w: u32, h: u32, name: String) -> Self {
        Self {
            init_x: x,
            init_y: y,
            x: x,
            y: y,
            w: w,
            h: h,
            speed: 0,
            color: Color::RED,
            name: name,
        }
    }
}

struct Model {
    width: u32,
    height: u32,
    objects: Vec<Box<dyn ProcessObject>>,
}

impl Model {
    pub fn process(&mut self) {
        for object in self.objects.iter_mut() {
            object.process(0.001, self.width, self.height);
        }
    }
    pub fn draw(&mut self, draw_handler: &mut RaylibDrawHandle) {
        for object in self.objects.iter_mut() {
            object.draw(draw_handler);
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();
    let ball1 = Ball::new(320, 240, 20, G, String::from("Ball 1"));
    let ball2 = Ball::new(400, 240, 20, 5.25, String::from("Ball 2"));
    let mut model = Model {
        width: 640,
        height: 480,
        objects: vec![Box::new(ball1), Box::new(ball2)],
    };

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        model.process();
        model.draw(&mut d);
        //    d.draw_circle(ball.x as i32, ball.y as i32, ball.radius, ball.color);
    }
}
