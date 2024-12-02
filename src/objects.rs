use super::quadtree;
use dyn_clone::DynClone;
use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle, Vector2},
};

pub trait ObjectTrait: std::fmt::Display + DynClone {
    fn draw(&self, draw_handler: &mut RaylibDrawHandle);

    fn set_coordinate(&mut self, new_vec: Vector2);
    fn set_color(&mut self, color: Color);
    fn set_speed(&mut self, speed: Vector2);
    fn set_acel(&mut self, acel: Vector2);

    fn update_coordinate(&mut self, new_vec: Vector2);
    fn update_speed(&mut self, speed: Vector2);
    fn update_acel(&mut self, acel: Vector2);

    fn get_box(&self) -> quadtree::QuadBox;
    fn get_coordinate(&self) -> Vector2;
    fn get_speed(&self) -> Vector2;
    fn get_acel(&self) -> Vector2;
}
#[derive(Clone)]
pub struct Rectangle {
    coordinate: Vector2,
    width: f32,
    height: f32,
    speed: Vector2,
    acel: Vector2,
    name: String,
    color: Color,
}

pub struct RectangleBuilder {
    coordinate: Vector2,
    width: f32,
    height: f32,
    speed: Vector2,
    acel: Vector2,
    name: String,
    color: Color,
}

impl RectangleBuilder {
    pub fn new() -> RectangleBuilder {
        RectangleBuilder {
            coordinate: Vector2 { x: 0.0, y: 0.0 },
            width: 0.0,
            height: 0.0,
            speed: Vector2 { x: 0.0, y: 0.0 },
            acel: Vector2 { x: 0.0, y: 0.0 },
            name: String::from("Object"),
            color: Color::BLACK,
        }
    }
    pub fn coordinate(mut self, x: f32, y: f32) -> RectangleBuilder {
        self.coordinate.x = x;
        self.coordinate.y = y;
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
    pub fn speed(mut self, speed: Vector2) -> RectangleBuilder {
        self.speed = speed;
        self
    }
    pub fn acel(mut self, acel: Vector2) -> RectangleBuilder {
        self.acel = acel;
        self
    }

    pub fn build(self) -> Rectangle {
        Rectangle {
            coordinate: self.coordinate,
            width: self.width,
            height: self.height,
            speed: self.speed,
            acel: self.acel,
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
            self.name, self.coordinate.x, self.coordinate.y, self.width, self.height
        )
    }
}

impl ObjectTrait for Rectangle {
    fn draw(&self, draw_handler: &mut RaylibDrawHandle) {
        draw_handler.draw_text(
            &self.name,
            self.coordinate.x as i32 + 5,
            self.coordinate.y as i32 + 5,
            7,
            Color::BLACK,
        );
        let rec = raylib::prelude::Rectangle {
            x: self.coordinate.x,
            y: self.coordinate.y,
            width: self.width,
            height: self.height,
        };
        draw_handler.draw_rectangle_lines_ex(rec, 3.0, self.color);
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    fn set_coordinate(&mut self, new_vec: Vector2) {
        self.coordinate.x = new_vec.x;
        self.coordinate.y = new_vec.y;
    }
    fn set_acel(&mut self, acel: Vector2) {
        self.acel = acel;
    }
    fn set_speed(&mut self, speed: Vector2) {
        self.speed = speed;
    }

    fn update_coordinate(&mut self, new_vec: Vector2) {
        self.coordinate.x += new_vec.x;
        self.coordinate.y += new_vec.y;
    }
    fn update_acel(&mut self, acel: Vector2) {
        self.acel += acel;
    }
    fn update_speed(&mut self, speed: Vector2) {
        self.speed += speed;
    }

    fn get_box(&self) -> quadtree::QuadBox {
        quadtree::QuadBox::new(
            self.coordinate.x,
            self.coordinate.y,
            self.width,
            self.height,
        )
    }
    fn get_acel(&self) -> Vector2 {
        self.acel
    }
    fn get_coordinate(&self) -> Vector2 {
        self.coordinate
    }
    fn get_speed(&self) -> Vector2 {
        self.speed
    }
}

#[derive(Clone)]
pub struct Circle {
    pub coordinate: Vector2,
    pub acel: Vector2,
    pub speed: Vector2,
    pub radius: f32,
    pub color: Color,
    name: String,
}
pub struct CircleBuilder {
    pub coordinate: Vector2,
    pub acel: Vector2,
    pub speed: Vector2,
    pub radius: f32,
    pub color: Color,
    name: String,
}
impl CircleBuilder {
    pub fn new() -> CircleBuilder {
        CircleBuilder {
            coordinate: Vector2 { x: 0.0, y: 0.0 },
            acel: Vector2 { x: 0.0, y: 0.0 },
            speed: Vector2 { x: 0.0, y: 0.0 },
            radius: 0.0,
            color: Color::RED,
            name: String::from("Object"),
        }
    }
    pub fn coordinate(mut self, x: f32, y: f32) -> CircleBuilder {
        self.coordinate.x = x;
        self.coordinate.y = y;
        self
    }
    pub fn name(mut self, name: &str) -> CircleBuilder {
        self.name = name.to_string();
        self
    }
    pub fn acel(mut self, acel: Vector2) -> CircleBuilder {
        self.acel = acel;
        self
    }
    pub fn speed(mut self, speed: Vector2) -> CircleBuilder {
        self.speed = speed;
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
            coordinate: self.coordinate,
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
            self.coordinate.x, self.coordinate.y, self.radius
        )
    }
}

impl ObjectTrait for Circle {
    fn draw(&self, draw_handler: &mut RaylibDrawHandle) {
        draw_handler.draw_text(
            &self.name,
            self.coordinate.x as i32 + 5,
            self.coordinate.y as i32 + 5,
            7,
            Color::BLACK,
        );
        draw_handler.draw_circle(
            self.coordinate.x as i32,
            self.coordinate.y as i32,
            self.radius,
            self.color,
        );
    }
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    fn set_coordinate(&mut self, new_vec: Vector2) {
        self.coordinate.x = new_vec.x + self.radius;
        self.coordinate.y = new_vec.y + self.radius;
    }
    fn set_acel(&mut self, acel: Vector2) {
        self.acel = acel;
    }
    fn set_speed(&mut self, speed: Vector2) {
        self.speed = speed;
    }
    fn update_coordinate(&mut self, new_vec: Vector2) {
        self.coordinate.x += new_vec.x;
        self.coordinate.y += new_vec.y;
    }
    fn update_acel(&mut self, acel: Vector2) {
        self.acel += acel;
    }
    fn update_speed(&mut self, speed: Vector2) {
        self.speed += speed
    }
    fn get_box(&self) -> quadtree::QuadBox {
        quadtree::QuadBox::new(
            self.coordinate.x - self.radius,
            self.coordinate.y - self.radius,
            self.radius * 2.0,
            self.radius * 2.0,
        )
    }
    fn get_acel(&self) -> Vector2 {
        self.acel
    }
    fn get_coordinate(&self) -> Vector2 {
        Vector2 {
            x: self.coordinate.x - self.radius,
            y: self.coordinate.y - self.radius,
        }
    }
    fn get_speed(&self) -> Vector2 {
        self.speed
    }
}
