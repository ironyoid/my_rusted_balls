use crate::{objects::MovingObject, quadtree, TreeObject};
use raylib::math::Vector2;
use std::time::SystemTime;

pub struct BaseMovementModel;
pub struct BaseCollisionModel;

pub trait MovementModel {
    fn process_movement(&mut self, object: &mut Box<impl MovingObject>, time_delta: f32);
}

pub trait CollisionModel {
    fn process_collision(
        &mut self,
        object: &mut Box<impl MovingObject>,
        pen_vectors: &Vec<Option<Vector2>>,
        time_delta: f32,
    );
}

impl MovementModel for BaseMovementModel {
    fn process_movement(&mut self, object: &mut Box<impl MovingObject>, time_delta: f32) {
        let speed = object.get_speed();
        let acel = object.get_acel();
        object.update_speed(object.get_acel() * time_delta);
        object.set_acel(acel);
        let delta_y = speed.y * time_delta + acel.y * time_delta.powf(2.0) / 2.0;
        object.update_coordinate(Vector2 {
            x: speed.x * time_delta,
            y: delta_y,
        });
    }
}

pub struct MouseMovementModel {
    mouse_position: Vector2,
}

impl MovementModel for MouseMovementModel {
    fn process_movement(&mut self, object: &mut Box<impl MovingObject>, time_delta: f32) {
        let delta_coord = self.mouse_position - object.get_coordinate();
        let mut speed = object.get_speed();
        speed += delta_coord * 2.0;
        object.update_coordinate(Vector2 {
            x: speed.x * time_delta,
            y: speed.y * time_delta,
        });
    }
}

impl MouseMovementModel {
    pub fn new() -> Self {
        MouseMovementModel {
            mouse_position: Vector2 { x: 0.0, y: 0.0 },
        }
    }
    pub fn set_mouse_position(&mut self, coords: Vector2) {
        self.mouse_position = coords;
    }
}

impl CollisionModel for BaseCollisionModel {
    fn process_collision(
        &mut self,
        object: &mut Box<impl MovingObject>,
        pen_vectors: &Vec<Option<Vector2>>,
        time_delta: f32,
    ) {
        for pen in pen_vectors.iter() {
            if let Some(val) = pen {
                let nrm = val.normalized();
                let speed = object.get_speed();
                let coords = object.get_coordinate();
                let line = Vector2 { x: nrm.y, y: nrm.x };

                object.set_speed(Vector2 {
                    x: (speed.x * line.x + speed.y * line.y) * line.x
                        - (speed.x * nrm.x + speed.y * nrm.y) * nrm.x,
                    y: (speed.x * line.x + speed.y * line.y) * line.y
                        - (speed.x * nrm.x + speed.y * nrm.y) * nrm.y,
                });
                object.set_coordinate(Vector2 {
                    x: coords.x + val.x,
                    y: coords.y + val.y,
                });
            }
        }
    }
}

pub struct Model<T: MovementModel, E: CollisionModel> {
    m_model: T,
    c_model: E,
    screen_width: f32,
    screen_height: f32,
    last_time: f64,
    period: f64,
}

impl<T: MovementModel, E: CollisionModel> Model<T, E> {
    pub fn new(m_model: T, c_model: E, width: f32, height: f32, period: f64) -> Self {
        Model {
            m_model: m_model,
            c_model: c_model,
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
        mov_objects: &mut Vec<Box<impl TreeObject + MovingObject>>,
        obj_tree: &mut quadtree::QuadTree,
        time_delta: f32,
    ) {
        for obj in mov_objects.iter_mut() {
            let mut pen_vector = obj_tree.query(obj);
            pen_vector.push(self.screen_collision(obj));
            self.c_model.process_collision(obj, &pen_vector, time_delta);
            self.m_model.process_movement(obj, time_delta);
        }
    }

    pub fn run(
        &mut self,
        mov_objects: &mut Vec<Box<impl MovingObject + TreeObject>>,
        obj_tree: &mut quadtree::QuadTree,
    ) {
        let curr_time = Self::get_time_s();
        let time_delta = curr_time - self.last_time;
        if time_delta >= self.period {
            self.last_time = curr_time;
            self.process(mov_objects, obj_tree, time_delta as f32);
        }
    }

    fn screen_collision(
        &mut self,
        object: &mut Box<impl MovingObject + TreeObject>,
    ) -> Option<Vector2> {
        let bx = object.get_box();
        let mut ret = Vector2 { x: 0.0, y: 0.0 };
        if bx.get_lefttop().x <= 0.0 {
            ret.x = -bx.get_lefttop().x;
        }
        if bx.get_right_x() >= self.screen_width {
            ret.x = self.screen_width - bx.get_right_x();
        }
        if bx.get_lefttop().y <= 0.0 {
            ret.y = -bx.get_lefttop().y;
        }
        if bx.get_bottom_y() >= self.screen_height {
            ret.y = self.screen_height - bx.get_bottom_y();
        }
        if ret.x != 0.0 || ret.y != 0.0 {
            return Some(ret);
        }
        None
    }

    pub fn get_m_model(&mut self) -> &mut T {
        &mut self.m_model
    }

    pub fn get_c_model(&mut self) -> &mut E {
        &mut self.c_model
    }
}
