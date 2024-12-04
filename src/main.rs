#![allow(dead_code, unused_variables)]
#![feature(extract_if)]
use objects::ObjectTrait;
use raylib::prelude::*;
mod objects;
mod physics;
mod quadtree;

fn main() {
    const G: f32 = 5000.0;
    const WINDOW_WIDTH: u32 = 640;
    const WINDOW_HEIGHT: u32 = 480;
    const MODEL_PERIOD: f64 = 0.005;

    let mut tree = quadtree::QuadTree::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
    let mut elems: Vec<Box<dyn ObjectTrait>> = Vec::new();
    elems.push(Box::new(
        objects::RectangleBuilder::new()
            .coordinate(100.0, 150.0)
            .size(60.0, 60.0)
            .name("box 1")
            .color(Color::REBECCAPURPLE)
            .build(),
    ));
    elems.push(Box::new(
        objects::RectangleBuilder::new()
            .coordinate(400.0, 150.0)
            .size(60.0, 60.0)
            .name("box 2")
            .color(Color::REBECCAPURPLE)
            .build(),
    ));
    elems.push(Box::new(
        objects::RectangleBuilder::new()
            .coordinate(300.0, 300.0)
            .size(70.0, 70.0)
            .name("box 3")
            .color(Color::REBECCAPURPLE)
            .build(),
    ));
    for n in elems.iter() {
        tree.add(n);
    }

    let move_elem: Box<dyn ObjectTrait> = Box::new(
        objects::CircleBuilder::new()
            .coordinate(100.0, 100.0)
            .radius(30.0)
            .acel(Vector2 { x: 500.0, y: G })
            .color(Color::RED)
            .build(),
    );
    let move_elems = &mut vec![move_elem];

    let mut phy = physics::Model::new(
        physics::MouseMovementModel::new(),
        physics::BaseCollisionModel {},
        WINDOW_WIDTH as f32,
        WINDOW_HEIGHT as f32,
        MODEL_PERIOD,
    );
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .title("Bouncing Balls")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        phy.get_m_model().set_mouse_position(d.get_mouse_position());
        d.clear_background(Color::WHITE);
        phy.run(move_elems, &mut tree);
        move_elems[0].draw(&mut d);
        for n in elems.iter() {
            n.draw(&mut d);
        }
    }
}
