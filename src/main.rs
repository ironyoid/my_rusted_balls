#![allow(dead_code, unused_variables)]
use my_rusted_balls::objects::*;
use my_rusted_balls::physics::*;
use my_rusted_balls::quadtree::*;
use raylib::prelude::*;

fn main() {
    const Y_AXIS_ACEL: f32 = 5000.0;
    const X_AXIS_ACEL: f32 = 1000.0;

    const WINDOW_WIDTH: u32 = 1024;
    const WINDOW_HEIGHT: u32 = 1024;
    const MODEL_PERIOD: f64 = 0.01;
    const MAX_REC_WIDTH: u32 = 70;
    const MIN_REC_WIDTH: u32 = 30;
    const MAX_REC_HEIGHT: u32 = 70;
    const MIN_REC_HEIGHT: u32 = 30;

    let mut tree = QuadTree::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
    let elems = my_rusted_balls::gen_vec_of_objects(
        50,
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

    let move_elem = Box::new(
        CircleBuilder::new()
            .coordinate(100.0, 100.0)
            .radius(30.0)
            .acel(Vector2 {
                x: X_AXIS_ACEL,
                y: Y_AXIS_ACEL,
            })
            .color(Color::RED)
            .build(),
    );
    let move_elems = &mut vec![move_elem];

    let mut phy = PhysicsModel::new(
        BaseMovementModel {},
        BaseCollisionModel {},
        WINDOW_WIDTH as f32,
        WINDOW_HEIGHT as f32,
        MODEL_PERIOD,
    );
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .title("My Rusted Balls")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        //phy.get_m_model().set_mouse_position(d.get_mouse_position());
        d.clear_background(Color::WHITE);
        tree.draw_tree(&mut d);
        phy.run(move_elems, &mut tree);
        move_elems[0].draw(&mut d);
    }
}
