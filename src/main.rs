#![allow(dead_code, unused_variables)]
#![feature(extract_if)]
use objects::*;
use quadtree::*;
use raylib::prelude::*;
mod objects;
mod physics;
mod quadtree;
use rand::Rng;
use raylib::math::Rectangle;
use std::time::{Duration, Instant};

fn gen_vec_of_objects(size: u32, width: u32, height: u32) -> Vec<Box<dyn TreeObject>> {
    let mut ret: Vec<Box<dyn TreeObject>> = Vec::new();
    let mut rng = rand::thread_rng();
    const MAX_REC_WIDTH: u32 = 30;
    const MAX_REC_HEIGHT: u32 = 30;
    for n in 0..size {
        ret.push(Box::new(
            RectangleBuilder::new()
                .coordinate(
                    rng.gen_range(0..width) as f32,
                    rng.gen_range(0..height) as f32,
                )
                .size(
                    rng.gen_range(5..MAX_REC_WIDTH) as f32,
                    rng.gen_range(5..MAX_REC_HEIGHT) as f32,
                )
                .color(Color {
                    r: rng.gen_range(0..255),
                    g: rng.gen_range(0..255),
                    b: rng.gen_range(0..255),
                    a: 255,
                })
                .build(),
        ));
    }
    ret
}

fn brut_force_find(elems: &Vec<Box<dyn TreeObject>>, target: QuadBox) -> Vec<Option<Vector2>> {
    let mut ret: Vec<Option<Vector2>> = Vec::new();
    for n in elems.iter() {
        let tmp = n.get_box();
        let bx = tmp.minkowski_difference(&target);
        if n.get_box().intersects(&target) {
            ret.push(tmp.pen_vector(&target, &bx));
        }
    }
    ret
}

fn main() {
    const G: f32 = 5000.0;
    const WINDOW_WIDTH: u32 = 1024;
    const WINDOW_HEIGHT: u32 = 1024;
    const MODEL_PERIOD: f64 = 0.01;

    let mut tree = quadtree::QuadTree::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
    //let mut elems: Vec<Box<dyn TreeObject>> = Vec::new();
    // elems.push(Box::new(
    //     RectangleBuilder::new()
    //         .coordinate(100.0, 150.0)
    //         .size(60.0, 60.0)
    //         .name("box 1")
    //         .color(Color::REBECCAPURPLE)
    //         .build(),
    // ));
    // elems.push(Box::new(
    //     RectangleBuilder::new()
    //         .coordinate(400.0, 150.0)
    //         .size(60.0, 60.0)
    //         .name("box 2")
    //         .color(Color::REBECCAPURPLE)
    //         .build(),
    // ));
    // elems.push(Box::new(
    //     RectangleBuilder::new()
    //         .coordinate(300.0, 300.0)
    //         .size(70.0, 70.0)
    //         .name("box 3")
    //         .color(Color::REBECCAPURPLE)
    //         .build(),
    // ));
    let elems = gen_vec_of_objects(100000, WINDOW_WIDTH, WINDOW_HEIGHT);
    for n in elems.iter() {
        tree.add(n);
    }
    //tree.print();
    // let move_elem = Box::new(
    //     CircleBuilder::new()
    //         .coordinate(100.0, 100.0)
    //         .radius(30.0)
    //         .acel(Vector2 { x: 500.0, y: G })
    //         .color(Color::RED)
    //         .build(),
    // );
    //let move_elems = &mut vec![move_elem];

    // let mut phy = physics::Model::new(
    //     physics::BaseMovementModel {},
    //     physics::BaseCollisionModel {},
    //     WINDOW_WIDTH as f32,
    //     WINDOW_HEIGHT as f32,
    //     MODEL_PERIOD,
    // );
    // let (mut rl, thread) = raylib::init()
    //     .size(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
    //     .title("My Rusted Balls")
    //     .build();

    let mut target = Box::new(
        RectangleBuilder::new()
            .coordinate(100.0, 100.0)
            .size(50.0, 50.0)
            .build(),
    );

    const ATTEMPTS: u32 = 1000;
    let mut rng = rand::thread_rng();
    let mut total_brut: Duration = Duration::new(0, 0);
    let mut total_tree: Duration = Duration::new(0, 0);
    for n in 0..ATTEMPTS {
        target.set_coordinate(Vector2 {
            x: rng.gen_range(0..WINDOW_WIDTH - 50) as f32,
            y: rng.gen_range(0..WINDOW_HEIGHT - 50) as f32,
        });
        let now = Instant::now();
        brut_force_find(&elems, target.get_box());
        total_brut += now.elapsed();
        let now = Instant::now();
        tree.query(&target);
        total_tree += now.elapsed();
    }
    println!(
        "Total brut: {:.3?} Total tree: {:.3?}",
        total_brut / ATTEMPTS,
        total_tree / ATTEMPTS
    );

    //}
    // while !rl.window_should_close() {
    //     let mut d = rl.begin_drawing(&thread);
    //     //phy.get_m_model().set_mouse_position(d.get_mouse_position());
    //     d.clear_background(Color::WHITE);
    //     tree.draw_tree(&mut d);
    //     let boxes = tree.get_boxes();
    //     for n in boxes.iter() {
    //         if let Some(x) = n {
    //             let rec = Rectangle {
    //                 x: x.get_lefttop().x,
    //                 y: x.get_lefttop().y,
    //                 width: x.get_size().x,
    //                 height: x.get_size().y,
    //             };
    //             d.draw_rectangle_lines_ex(rec, 1.0, Color::ROYALBLUE);
    //         }
    //     }
    //     //phy.run(move_elems, &mut tree);
    //     //move_elems[0].draw(&mut d);
    //     //for n in elems.iter() {
    //     //n.draw(&mut d);
    //     //}
    // }
}
