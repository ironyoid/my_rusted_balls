#![allow(dead_code, unused_variables)]
use my_rusted_balls::objects::*;
use my_rusted_balls::quadtree::*;
use rand::Rng;
use raylib::prelude::*;
use std::time::{Duration, Instant};

#[cfg(test)]

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

#[test]
fn quadtree_benchmark() {
    const WINDOW_WIDTH: u32 = 1024;
    const WINDOW_HEIGHT: u32 = 1024;
    const NUM_OF_OBJECTS: u32 = 100000;
    const MAX_REC_WIDTH: u32 = 100;
    const MIN_REC_WIDTH: u32 = 10;
    const MAX_REC_HEIGHT: u32 = 100;
    const MIN_REC_HEIGHT: u32 = 10;
    let mut tree = QuadTree::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
    let elems = my_rusted_balls::gen_vec_of_objects(
        NUM_OF_OBJECTS,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        MIN_REC_WIDTH,
        MAX_REC_WIDTH,
        MIN_REC_HEIGHT,
        MAX_REC_HEIGHT,
    );
    for n in elems.iter() {
        tree.add(n);
    }
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
}
