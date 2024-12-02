use crate::quadtree::ObjectTrait;

struct BaseModel;

trait ModelTrait {
    fn process(&mut self, object: &Box<dyn ObjectTrait>, time_delta: f32);
}

impl ModelTrait for BaseModel {
    fn process(&mut self, object: &Box<dyn ObjectTrait>, time_delta: f32) {
        println!(
            "speed: {},{} current: {},{} acel: {}",
            self.speed.x, self.speed.y, self.current.x, self.current.y, self.acel.y
        );
        self.speed = self.speed + self.acel * time_delta;
        self.acel.x = self.acel.x / 1.15;
        let delta_y = self.speed.y * time_delta + self.acel.y * time_delta.powf(2.0) / 2.0;
        self.current.y = self.current.y + delta_y;
        self.current.x = self.current.x + self.speed.x * time_delta;

        if self.detect_collision(w, h) {
            self.speed = -self.speed * 0.9;
        }
        if self.detect_collision(w, h) {
            self.y = h as f64 - self.radius;
            self.speed = -self.speed;
            self.y -= delta_y;
        }
    }
}

struct Physics<T: ModelTrait> {
    model: T,
}

impl<T: ModelTrait> Physics<T> {
    fn process(&mut self, mov_objects: &mut Vec<Box<dyn ObjectTrait>>, time_delta: f32) {
        for obj in mov_objects.iter_mut() {
            self.model.process(&obj, time_delta);
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
