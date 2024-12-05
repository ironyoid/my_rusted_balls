#![feature(extract_if)]
#![allow(dead_code, unused_variables)]
pub mod objects;
pub mod physics;
pub mod quadtree;
pub use rand::Rng;

pub fn gen_vec_of_objects(
    size: u32,
    width: u32,
    height: u32,
    min_rec_width: u32,
    max_rec_width: u32,
    min_rec_height: u32,
    max_rec_height: u32,
) -> Vec<Box<dyn quadtree::TreeObject>> {
    let mut ret: Vec<Box<dyn quadtree::TreeObject>> = Vec::new();
    let mut rng = rand::thread_rng();
    for n in 0..size {
        ret.push(Box::new(
            objects::RectangleBuilder::new()
                .coordinate(
                    rng.gen_range(0..width) as f32,
                    rng.gen_range(0..height) as f32,
                )
                .size(
                    rng.gen_range(min_rec_width..max_rec_width) as f32,
                    rng.gen_range(min_rec_height..max_rec_height) as f32,
                )
                .build(),
        ));
    }
    ret
}
