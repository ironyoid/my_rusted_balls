use crate::{objects::ObjectTrait, quadtree};
use raylib::math::Vector2;
use std::time::SystemTime;

pub struct BaseModel;

pub trait ModelTrait {
    fn process(
        &mut self,
        object: &mut Box<dyn ObjectTrait>,
        pen_vectors: &Vec<Option<Vector2>>,
        time_delta: f32,
    );
}

impl ModelTrait for BaseModel {
    fn process(
        &mut self,
        object: &mut Box<dyn ObjectTrait>,
        pen_vectors: &Vec<Option<Vector2>>,
        time_delta: f32,
    ) {
        // println!(
        //     "speed: {},{} current: {},{} acel: {},{}",
        //     speed.x,
        //     speed.y,
        //     object.get_coordinate().x,
        //     object.get_coordinate().y,
        //     acel.x,
        //     acel.y
        // );
        for pen in pen_vectors.iter() {
            if let Some(val) = pen {
                let nrm = val.normalized();
                //println!("pen: {},{} norm: {},{}", val.x, val.y, nrm.x, nrm.y);
                let speed = object.get_speed();
                let coords = object.get_coordinate();
                let line = Vector2 { x: nrm.y, y: nrm.x };
                let dp_a = speed.x * line.x + speed.y * line.y;
                let pr_a = Vector2 {
                    x: dp_a * line.x,
                    y: dp_a * line.y,
                };
                let dp_b = speed.x * nrm.x + speed.y * nrm.y;
                let pr_b = Vector2 {
                    x: dp_b * nrm.x,
                    y: dp_b * nrm.y,
                };
                let new_speed = Vector2 {
                    x: pr_a.x - pr_b.x,
                    y: pr_a.y - pr_b.y,
                };
                let new_coords = Vector2 {
                    x: coords.x + val.x,
                    y: coords.y + val.y,
                };
                // println!("new_speed: {},{}", new_speed.x, new_speed.y);
                // println!("new_coords: {},{}", new_coords.x, new_coords.y);
                object.set_speed(new_speed);
                object.set_coordinate(new_coords);
            }

            let speed = object.get_speed();
            let acel = object.get_acel();
            object.update_speed(object.get_acel() * time_delta);
            object.set_acel(acel);
            let delta_y = speed.y * time_delta + acel.y * time_delta.powf(2.0) / 2.0;
            object.update_coordinate(Vector2 {
                x: speed.x * time_delta,
                y: delta_y,
            });
            //object.set_acel(new_acel);
        }
        // println!(
        //     "speed: {},{} current: {},{} acel: {}",
        //     self.speed.x, self.speed.y, self.current.x, self.current.y, self.acel.y
        // );
        // self.speed = self.speed + self.acel * time_delta;
        // self.acel.x = self.acel.x / 1.15;
        // let delta_y = self.speed.y * time_delta + self.acel.y * time_delta.powf(2.0) / 2.0;
        // self.current.y = self.current.y + delta_y;
        // self.current.x = self.current.x + self.speed.x * time_delta;

        // if self.detect_collision(w, h) {
        //     self.speed = -self.speed * 0.9;
        // }
        // if self.detect_collision(w, h) {
        //     self.y = h as f64 - self.radius;
        //     self.speed = -self.speed;
        //     self.y -= delta_y;
        // }
    }
}

pub struct Physics<T: ModelTrait> {
    model: T,
    screen_width: f32,
    screen_height: f32,
    last_time: f64,
    period: f64,
}

impl<T: ModelTrait> Physics<T> {
    pub fn new(model: T, width: f32, height: f32, period: f64) -> Self {
        Physics {
            model: model,
            screen_height: height,
            screen_width: width,
            last_time: Self::get_time_s(),
            period: period,
        }
    }

    fn get_time_s() -> f64 {
        let duration_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        duration_since_epoch.as_millis() as f64 / 1000.0
    }

    fn process(
        &mut self,
        mov_objects: &mut Vec<Box<dyn ObjectTrait>>,
        obj_tree: &mut quadtree::QuadTree,
        time_delta: f32,
    ) {
        for obj in mov_objects.iter_mut() {
            let mut pen_vector: Vec<Option<Vector2>> = Vec::new();
            let tmp = self.screen_collision(obj);
            let mut st_obj = obj_tree.query(obj);
            pen_vector.append(&mut st_obj);
            pen_vector.push(tmp);

            self.model.process(obj, &pen_vector, time_delta);
        }
    }

    pub fn run(
        &mut self,
        mov_objects: &mut Vec<Box<dyn ObjectTrait>>,
        obj_tree: &mut quadtree::QuadTree,
    ) {
        let curr_time = Self::get_time_s();
        let time_delta = curr_time - self.last_time;
        //println!("time: {} {}", curr_time, Self::get_time_s());

        if time_delta >= self.period {
            self.last_time = curr_time;
            self.process(mov_objects, obj_tree, time_delta as f32);
        }
    }

    fn screen_collision(&mut self, object: &mut Box<dyn ObjectTrait>) -> Option<Vector2> {
        let bx = object.get_box();
        if bx.get_lefttop().x <= 0.0
            && bx.get_lefttop().y >= 0.0
            && bx.get_bottom_y() <= self.screen_height
        {
            //return Some(Vector2 { x: 1.0, y: 0.0 });
            return Some(Vector2 {
                x: -bx.get_lefttop().x,
                y: 0.0,
            });
        }
        if bx.get_right_x() >= self.screen_width
            && bx.get_lefttop().y >= 0.0
            && bx.get_bottom_y() <= self.screen_height
        {
            //return Some(Vector2 { x: -1.0, y: 0.0 });

            return Some(Vector2 {
                x: self.screen_width - bx.get_right_x(),
                y: 0.0,
            });
        }
        if bx.get_lefttop().y <= 0.0
            && bx.get_lefttop().x >= 0.0
            && bx.get_right_x() <= self.screen_width
        {
            //return Some(Vector2 { x: 0.0, y: 1.0 });
            return Some(Vector2 {
                x: 0.0,
                y: -bx.get_lefttop().y,
            });
        }
        if bx.get_bottom_y() >= self.screen_height
            && bx.get_lefttop().x >= 0.0
            && bx.get_right_x() <= self.screen_width
        {
            //return Some(Vector2 { x: 0.0, y: -1.0 });
            return Some(Vector2 {
                x: 0.0,
                y: self.screen_height - bx.get_bottom_y(),
            });
        }

        if bx.get_lefttop().x <= 0.0 && bx.get_lefttop().y <= 0.0 {
            //return Some(Vector2 { x: 0.0, y: -1.0 });
            return Some(Vector2 {
                x: -bx.get_lefttop().x,
                y: -bx.get_lefttop().y,
            });
        }
        if bx.get_right_x() >= self.screen_width && bx.get_lefttop().y <= 0.0 {
            //return Some(Vector2 { x: 0.0, y: -1.0 });
            return Some(Vector2 {
                x: self.screen_width - bx.get_right_x(),
                y: -bx.get_lefttop().y,
            });
        }
        if bx.get_bottom_y() >= self.screen_height && bx.get_right_x() >= self.screen_width {
            //return Some(Vector2 { x: 0.0, y: -1.0 });
            return Some(Vector2 {
                x: self.screen_width - bx.get_right_x(),
                y: self.screen_height - bx.get_bottom_y(),
            });
        }
        if bx.get_bottom_y() >= self.screen_height && bx.get_lefttop().x <= 0.0 {
            //return Some(Vector2 { x: 0.0, y: -1.0 });
            return Some(Vector2 {
                x: -bx.get_lefttop().x,
                y: self.screen_height - bx.get_bottom_y(),
            });
        }

        None
    }
}
