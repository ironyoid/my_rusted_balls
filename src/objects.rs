use super::quadtree;
use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle, Vector2},
};

#[derive(Clone)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub name: String,
    pub color: Color,
}

pub struct RectangleBuilder {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub name: String,
    pub color: Color,
}

impl RectangleBuilder {
    pub fn new() -> RectangleBuilder {
        RectangleBuilder {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            name: String::from("Object"),
            color: Color::BLACK,
        }
    }
    pub fn coordinate(mut self, x: f32, y: f32) -> RectangleBuilder {
        self.x = x;
        self.y = y;
        self
    }
    pub fn size(mut self, width: f32, height: f32) -> RectangleBuilder {
        self.width = width;
        self.height = height;
        self
    }
    pub fn name(mut self, name: &str) -> RectangleBuilder {
        self.name = name.to_string();
        self
    }
    pub fn color(mut self, color: Color) -> RectangleBuilder {
        self.color = color;
        self
    }
    pub fn build(self) -> Rectangle {
        Rectangle {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            name: self.name,
            color: self.color,
        }
    }
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({}:[{}, {}, {}, {}])",
            self.name, self.x, self.y, self.width, self.height
        )
    }
}

impl quadtree::ObjectTrait for Rectangle {
    fn get_box(&self) -> quadtree::QuadBox {
        quadtree::QuadBox::new(self.x, self.y, self.width, self.height)
    }
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    fn draw(&self, draw_handler: &mut RaylibDrawHandle) {
        draw_handler.draw_text(
            &self.name,
            self.x as i32 + 5,
            self.y as i32 + 5,
            7,
            Color::BLACK,
        );
        let rec = raylib::prelude::Rectangle {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        };
        draw_handler.draw_rectangle_lines_ex(rec, 3.0, self.color);
    }
    fn set_coordinate(&mut self, new_vec: Vector2) {
        self.x = new_vec.x;
        self.y = new_vec.y;
    }
    fn update_coordinate(&mut self, new_vec: Vector2) {
        self.x += new_vec.x;
        self.y += new_vec.y;
    }
}

#[derive(Clone)]
pub struct Circle {
    pub init: Vector2,
    pub current: Vector2,
    pub acel: Vector2,
    pub speed: Vector2,
    pub radius: f32,
    pub color: Color,
    name: String,
}
pub struct CircleBuilder {
    pub init: Vector2,
    pub current: Vector2,
    pub acel: Vector2,
    pub speed: Vector2,
    pub radius: f32,
    pub color: Color,
    pub name: String,
}
impl CircleBuilder {
    pub fn new() -> CircleBuilder {
        CircleBuilder {
            init: Vector2 { x: 0.0, y: 0.0 },
            current: Vector2 { x: 0.0, y: 0.0 },
            acel: Vector2 { x: 0.0, y: 0.0 },
            speed: Vector2 { x: 0.0, y: 0.0 },
            radius: 0.0,
            color: Color::RED,
            name: String::from("Object"),
        }
    }
    pub fn coordinate(mut self, x: f32, y: f32) -> CircleBuilder {
        self.init.x = x;
        self.init.y = y;
        self.current = self.init;
        self
    }
    pub fn name(mut self, name: &str) -> CircleBuilder {
        self.name = name.to_string();
        self
    }
    pub fn acel(mut self, x: f32, y: f32) -> CircleBuilder {
        self.acel.x = x;
        self.acel.y = y;
        self
    }
    pub fn speed(mut self, x: f32, y: f32) -> CircleBuilder {
        self.speed.x = x;
        self.speed.y = y;
        self
    }
    pub fn radius(mut self, radius: f32) -> CircleBuilder {
        self.radius = radius;
        self
    }
    pub fn color(mut self, color: Color) -> CircleBuilder {
        self.color = color;
        self
    }
    pub fn build(self) -> Circle {
        Circle {
            init: self.init,
            current: self.current,
            acel: self.acel,
            speed: self.speed,
            radius: self.radius,
            color: self.color,
            name: self.name,
        }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "([{}, {}, {}]])",
            self.current.x, self.current.y, self.radius
        )
    }
}

impl quadtree::ObjectTrait for Circle {
    fn get_box(&self) -> quadtree::QuadBox {
        quadtree::QuadBox::new(
            self.current.x - self.radius,
            self.current.y - self.radius,
            self.radius * 2.0,
            self.radius * 2.0,
        )
    }
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    fn draw(&self, draw_handler: &mut RaylibDrawHandle) {
        draw_handler.draw_text(
            &self.name,
            self.current.x as i32 + 5,
            self.current.y as i32 + 5,
            7,
            Color::BLACK,
        );
        draw_handler.draw_circle(
            self.current.x as i32,
            self.current.y as i32,
            self.radius,
            self.color,
        );
    }
    fn set_coordinate(&mut self, new_vec: Vector2) {
        self.current.x = new_vec.x;
        self.current.y = new_vec.y;
    }
    fn update_coordinate(&mut self, new_vec: Vector2) {
        self.current.x += new_vec.x;
        self.current.y += new_vec.y;
    }
}
