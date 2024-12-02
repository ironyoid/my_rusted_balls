#![allow(dead_code, unused_variables)]
#![feature(extract_if)]
use objects::ObjectTrait;
use raylib::prelude::*;
mod objects;
mod physics;
mod quadtree;
trait ProcessObject {
    fn process(&mut self, time: f32, w: u32, h: u32);
    fn draw(&self, draw_handler: &mut RaylibDrawHandle);
    fn detect_collision(&mut self, w: u32, h: u32) -> bool;
}

// impl ProcessObject for objects::Circle {
//     fn process(&mut self, time: f32, w: u32, h: u32) {
//         println!(
//             "speed: {},{} current: {},{} acel: {}",
//             self.speed.x, self.speed.y, self.current.x, self.current.y, self.acel.y
//         );
//         self.speed = self.speed + self.acel * time;
//         self.acel.x = self.acel.x / 1.15;
//         let delta_y = self.speed.y * time + self.acel.y * time.powf(2.0) / 2.0;
//         self.current.y = self.current.y + delta_y;
//         self.current.x = self.current.x + self.speed.x * time;

//         if self.detect_collision(w, h) {
//             self.speed = -self.speed * 0.9;
//         }
//         // if self.detect_collision(w, h) {
//         //     self.y = h as f64 - self.radius;
//         //     self.speed = -self.speed;
//         //     self.y -= delta_y;
//         // }
//     }
//     fn detect_collision(&mut self, w: u32, h: u32) -> bool {
//         let tmp = self.current + self.radius;
//         if tmp.x >= w as f32 && tmp.y > 0.0 && tmp.y < h as f32 {
//             self.current.x = w as f32 - self.radius;
//             self.speed.y = -self.speed.y;
//             return true;
//         }
//         if tmp.x < 0.0 && tmp.y > 0.0 && tmp.y < h as f32 {
//             self.current.x = self.radius;
//             self.speed.y = -self.speed.y;
//             return true;
//         }
//         if tmp.y >= h as f32 && tmp.x > 0.0 && tmp.x < w as f32 {
//             self.current.y = h as f32 - self.radius;
//             self.speed.x = -self.speed.x;
//             return true;
//         }
//         if tmp.y <= 0.0 && tmp.x > 0.0 && tmp.x < w as f32 {
//             self.current.y = self.radius;
//             self.speed.x = -self.speed.x;
//             return true;
//         }
//         false
//     }
//     fn draw(&self, draw_handler: &mut RaylibDrawHandle) {
//         draw_handler.draw_circle(
//             self.current.x as i32,
//             self.current.y as i32,
//             self.radius as f32,
//             self.color,
//         );
//     }
// }

// struct Model {
//     width: u32,
//     height: u32,
//     last_time: f64,
//     period: f64,
//     objects: Vec<Box<dyn ProcessObject>>,
// }

// impl Model {
//     pub fn process(&mut self) {
//         let curr_time = get_time_s();
//         let time_delta = curr_time - self.last_time;
//         //println!("time: {}", time_delta);
//         if time_delta >= self.period {
//             self.last_time = curr_time;
//             for object in self.objects.iter_mut() {
//                 object.process(self.period as f32, self.width, self.height);
//             }
//         }
//     }
//     pub fn draw(&mut self, draw_handler: &mut RaylibDrawHandle) {
//         for object in self.objects.iter_mut() {
//             object.draw(draw_handler);
//         }
//     }
// }

// fn get_time_s() -> f32 {
//     let duration_since_epoch = SystemTime::now()
//         .duration_since(SystemTime::UNIX_EPOCH)
//         .unwrap();
//     duration_since_epoch.as_millis() as f32 / 1000.0
// }

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
            .build(),
    ));
    elems.push(Box::new(
        objects::RectangleBuilder::new()
            .coordinate(400.0, 150.0)
            .size(60.0, 60.0)
            .name("box 2")
            .build(),
    ));
    elems.push(Box::new(
        objects::RectangleBuilder::new()
            .coordinate(300.0, 300.0)
            .size(70.0, 70.0)
            .name("box 3")
            .build(),
    ));
    // elems.push(Box::new(
    //     objects::RectangleBuilder::new()
    //         .coordinate(50.0, 100.0)
    //         .size(70.0, 70.0)
    //         .name("box 3")
    //         .build(),
    // ));

    let move_elem: Box<dyn ObjectTrait> = Box::new(
        objects::CircleBuilder::new()
            .coordinate(100.0, 100.0)
            .radius(30.0)
            .acel(Vector2 { x: 500.0, y: G })
            .name("elem")
            .color(Color::RED)
            .build(),
    );
    let move_elems = &mut vec![move_elem];
    for n in elems.iter() {
        tree.add(n);
    }
    tree.print();
    let model = physics::BaseModel {};
    let mut phy = physics::Physics::new(
        model,
        WINDOW_WIDTH as f32,
        WINDOW_HEIGHT as f32,
        MODEL_PERIOD,
    );

    // let speed = Vector2 { x: 20.0, y: 20.0 };
    // let nrm = Vector2 { x: 0.0, y: -1.0 };
    // let line = Vector2 { x: nrm.y, y: nrm.x };
    // let dp_a = speed.x * nrm.x + speed.y * nrm.y;
    // println!("dp_a: {}", dp_a);
    // let pr_a = Vector2 {
    //     x: dp_a * nrm.x,
    //     y: dp_a * nrm.y,
    // };
    // println!("pr_a: {},{}", pr_a.x, pr_a.y);
    // let dp_b = speed.x * line.x + speed.y * line.y;
    // println!("dp_b: {}", dp_b);
    // let pr_b = Vector2 {
    //     x: dp_b * line.x,
    //     y: dp_b * line.y,
    // };
    // println!("pr_b: {},{}", pr_b.x, pr_b.y);
    // let new_speed = Vector2 {
    //     x: pr_a.x - pr_b.x,
    //     y: pr_a.y - pr_b.y,
    // };
    // println!("new_speed: {},{}", new_speed.x, new_speed.y);

    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .title("Bouncing Balls")
        .build();
    // let ball1 = BallBuilder::new()
    //     .pos(20 as f32, (WINDOW_HEIGHT / 2) as f32)
    //     .radius(20.0)
    //     .acel(9000.0, G)
    //     .build();

    // let ball2 = BallBuilder::new()
    //     .pos(WINDOW_WIDTH as f32 - 50.0, (WINDOW_HEIGHT / 2) as f32)
    //     .radius(40.0)
    //     .acel(20000.0, G)
    //     .color(Color::BLUE)
    //     .build();

    // let mut model = Model {
    //     width: WINDOW_WIDTH,
    //     height: WINDOW_HEIGHT,
    //     period: MODEL_PERIOD,
    //     last_time: get_time_s(),
    //     objects: vec![Box::new(ball1), Box::new(ball2)],
    // };
    // let mouse_vec = rl.get_mouse_position();
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        //move_elems[0].set_coordinate(d.get_mouse_position());
        phy.run(move_elems, &mut tree);
        //phy.process(, time_delta);
        // let pen = tree.query(&move_elem, &mut d);
        // for n in pen.iter() {
        //     //println!("[{},{}]", n.x, n.y);
        //     move_elem.update_coordinate(*n);
        // }
        // println!();
        move_elems[0].draw(&mut d);
        tree.draw_tree(&mut d);
        //model.process();
        //model.draw(&mut d);
    }
}
