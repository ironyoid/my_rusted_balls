use dyn_clone::DynClone;
use raylib::{
    color::Color,
    prelude::{RaylibDrawHandle, Vector2},
};
use std::collections::VecDeque;
pub trait ObjectTrait: std::fmt::Display + DynClone {
    fn get_box(&self) -> QuadBox;
    fn set_color(&mut self, color: Color);
    fn draw(&self, draw_handler: &mut RaylibDrawHandle);
    fn set_coordinate(&mut self, new_vec: Vector2);
}

pub struct QuadTree {
    root: Subtree,
    u_box: QuadBox,
    max_depth: u32,
    max_num_of_elems: usize,
}

struct Subtree(Option<Box<Node>>);

#[derive(Clone)]
pub struct QuadBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl std::fmt::Display for QuadBox {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[{}, {}, {}, {}]",
            self.x, self.y, self.width, self.height
        )
    }
}

impl QuadBox {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        QuadBox {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }

    fn get_center(&self) -> Vector2 {
        Vector2 {
            x: self.x + self.width / 2.0,
            y: self.y + self.height / 2.0,
        }
    }

    fn get_right_x(&self) -> f32 {
        self.x + self.width
    }

    fn get_bottom_y(&self) -> f32 {
        self.y + self.height
    }

    fn get_size(&self) -> Vector2 {
        Vector2 {
            x: self.width,
            y: self.height,
        }
    }

    fn contains(&self, u_box: &QuadBox) -> bool {
        u_box.x >= self.x
            && u_box.get_right_x() <= self.get_right_x()
            && u_box.y >= self.y
            && u_box.get_bottom_y() <= self.get_bottom_y()
    }

    fn intersects(&self, u_box: &QuadBox) -> bool {
        !(self.x >= u_box.get_right_x()
            || self.get_right_x() <= u_box.x
            || self.y >= u_box.get_bottom_y()
            || self.get_bottom_y() <= u_box.y)
    }
}

struct Node {
    values: Vec<Box<dyn ObjectTrait>>,
    children: [Subtree; 4],
}

impl Node {
    fn new() -> Self {
        Self {
            values: Vec::new(),
            children: [
                Subtree::new(),
                Subtree::new(),
                Subtree::new(),
                Subtree::new(),
            ],
        }
    }
    fn get_vec_of_refs(&self) -> Vec<&Subtree> {
        let mut vc: Vec<&Subtree> = Vec::new();
        for n in self.children.iter() {
            vc.push(n);
        }
        vc
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if write!(f, "[") == Ok(()) {
            for n in self.values.iter() {
                let tmp = write!(f, "{}", n);
                if tmp != Ok(()) {
                    return tmp;
                };
            }
        }
        write!(f, "]")
    }
}

trait MyTrait: DynClone {
    fn recite(&self);
}

impl MyTrait for String {
    fn recite(&self) {
        println!("{} ♫", self);
    }
}

impl Subtree {
    fn new() -> Self {
        Self(None)
    }

    fn draw_tree(&mut self, draw_handler: &mut RaylibDrawHandle) {
        match &mut self.0 {
            Some(x) => {
                for val in x.values.iter_mut() {
                    val.draw(draw_handler);
                    val.set_color(Color::BLUE);
                }
                for (idx, elem) in x.children.iter_mut().enumerate() {
                    elem.draw_tree(draw_handler);
                }
            }
            None => {}
        }
    }

    fn get_boxes(&mut self, init_box: &QuadBox, boxes: &mut Vec<Option<QuadBox>>) {
        match &mut self.0 {
            Some(x) => {
                boxes.push(Some(init_box.clone()));
                for (idx, elem) in x.children.iter_mut().enumerate() {
                    let bx_1 = Self::compute_box(init_box, idx as i32);
                    elem.get_boxes(&bx_1.as_ref().unwrap(), boxes);
                }
            }
            None => {}
        }
    }

    fn print_tree(&mut self) {
        let mut queue: VecDeque<Vec<&Subtree>> = VecDeque::new();
        let mut level: u32 = 0;
        match &self.0 {
            Some(x) => {
                let vc = x.get_vec_of_refs();
                queue.push_back(vc);
            }
            None => {}
        }
        println!("root: {}", self.0.as_ref().unwrap());

        while !queue.is_empty() {
            let mut vc: Vec<&Subtree> = Vec::new();
            let tmp = queue.pop_front().unwrap();
            print!("{} level_{}: ", tmp.len(), level);

            for n in tmp.iter() {
                match &n.0 {
                    Some(x) => {
                        print!("{}", x);
                        vc.append(&mut x.get_vec_of_refs());
                    }
                    None => {}
                }
            }
            if !vc.is_empty() {
                queue.push_back(vc);
            }

            println!();
            level += 1;
        }
    }

    fn add(
        &mut self,
        depth: u32,
        u_box: &QuadBox,
        elem: &Box<dyn ObjectTrait>,
        max_depth: u32,
        max_num: usize,
    ) {
        match &mut self.0 {
            Some(x) => {
                if x.children[0].0.is_none() {
                    if depth >= max_depth || x.values.len() < max_num {
                        x.values.push(dyn_clone::clone_box(&**elem));
                    } else {
                        Self::split(x, &u_box, &elem);
                        self.add(depth, u_box, elem, max_depth, max_num);
                    }
                } else {
                    let i = Self::get_quadrant(&u_box, &elem);
                    if i >= 0 {
                        let new_box = Self::compute_box(&u_box, i);
                        match new_box {
                            Some(bx) => {
                                x.children[i as usize].add(depth + 1, &bx, elem, max_depth, max_num)
                            }
                            None => println!("Compute box is None!"),
                        }
                    } else {
                        x.values.push(dyn_clone::clone_box(&**elem));
                    }
                }
            }
            None => {
                println!("Create root!");
                self.0 = Some(Box::new(Node::new()));
                self.add(depth, u_box, elem, max_depth, max_num);
            }
        }
    }

    fn split(node: &mut Box<Node>, u_box: &QuadBox, elem: &Box<dyn ObjectTrait>) {
        for n in node.children.iter_mut() {
            n.0 = Some(Box::new(Node::new()));
        }

        for (idx, child) in node.children.iter_mut().enumerate() {
            let mut tmp = node
                .values
                .extract_if(|x| {
                    let i = Self::get_quadrant(u_box, x);
                    i == idx as i32
                })
                .collect::<Vec<_>>();
            match &mut child.0 {
                Some(x) => x.values.append(&mut tmp),
                None => println!("Child is None"),
            }
        }
    }

    fn get_quadrant(node_box: &QuadBox, elem: &Box<dyn ObjectTrait>) -> i32 {
        let center = node_box.get_center();
        let elem_box = elem.get_box();
        if elem_box.get_right_x() < center.x {
            if elem_box.get_bottom_y() < center.y {
                return 0;
            } else if elem_box.y >= center.y {
                return 2;
            } else {
                return -1;
            }
        } else if elem_box.x >= center.x {
            if elem_box.get_bottom_y() < center.y {
                return 1;
            } else if elem_box.y >= center.y {
                return 3;
            } else {
                return -1;
            }
        } else {
            return -1;
        }
    }

    fn compute_box(node_box: &QuadBox, idx: i32) -> Option<QuadBox> {
        let origin = Vector2 {
            x: node_box.x,
            y: node_box.y,
        };
        let child_size = node_box.get_size() / 2.0;
        match idx {
            0 => Some(QuadBox::new(origin.x, origin.y, child_size.x, child_size.y)),
            1 => Some(QuadBox::new(
                origin.x + child_size.x,
                origin.y,
                child_size.x,
                child_size.y,
            )),
            2 => Some(QuadBox::new(
                origin.x,
                origin.y + child_size.y,
                child_size.x,
                child_size.y,
            )),
            3 => Some(QuadBox::new(
                origin.x + child_size.x,
                origin.y + child_size.y,
                child_size.x,
                child_size.y,
            )),
            _ => None,
        }
    }

    fn query(
        &mut self,
        init_box: &QuadBox,
        u_box: &QuadBox,
        ret_elems: &mut Vec<Box<dyn ObjectTrait>>,
    ) {
        if let Some(x) = &mut self.0 {
            for n in x.values.iter_mut() {
                if u_box.intersects(&n.get_box()) {
                    ret_elems.push(dyn_clone::clone_box(&**n));
                    n.set_color(Color::RED);
                }
            }

            for (idx, n) in x.children.iter_mut().enumerate() {
                if let Some(x) = &n.0 {
                    let child_box = Self::compute_box(init_box, idx as i32);
                    if let Some(y) = child_box {
                        if u_box.intersects(&y) {
                            n.query(&y, u_box, ret_elems);
                        }
                    }
                }
            }
        }
    }
}

impl QuadTree {
    const MAX_DEPTH: u32 = 16;
    const MAX_NUM_OF_ELEMS: usize = 2;
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            root: Subtree::new(),
            u_box: QuadBox::new(0.0, 0.0, width, height),
            max_depth: Self::MAX_DEPTH,
            max_num_of_elems: Self::MAX_NUM_OF_ELEMS,
        }
    }
    pub fn add(&mut self, elem: &Box<dyn ObjectTrait>) {
        self.root
            .add(0, &self.u_box, elem, self.max_depth, self.max_num_of_elems);
    }
    pub fn print(&mut self) {
        self.root.print_tree();
    }
    pub fn get_boxes(&mut self) -> Vec<Option<QuadBox>> {
        let mut ret: Vec<Option<QuadBox>> = Vec::new();
        self.root.get_boxes(&self.u_box, &mut ret);
        ret
    }
    pub fn draw_tree(&mut self, draw_handler: &mut RaylibDrawHandle) {
        self.root.draw_tree(draw_handler);
    }
    pub fn query(&mut self, elem: &Box<dyn ObjectTrait>) -> Vec<Box<dyn ObjectTrait>> {
        let mut ret: Vec<Box<dyn ObjectTrait>> = Vec::new();
        self.root.query(&self.u_box, &elem.get_box(), &mut ret);
        ret
    }
}
