struct Element {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    name: String,
}

struct Node {
    values: Vec<Box<Element>>,
    children: [Subtree; 4],
}

pub struct QuadTree {
    root: Subtree,
    u_box: QuadBox,
    max_depth: u32,
    max_num_of_elems: usize,
}

struct Subtree(Option<Box<Node>>);

struct QuadBox {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl QuadBox {
    fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        QuadBox {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }
}

impl Subtree {
    fn new() -> Self {
        Self(None)
    }
    fn add(&mut self, depth: u32, u_box: QuadBox, elem: Element, max_depth: u32, max_num: usize) {
        match &mut self.0 {
            Some(x) => {
                if x.children[0].0.is_none() {
                    if depth >= max_depth || x.values.len() < max_num {
                        x.values.push(Box::new(elem));
                    } else {
                    }
                } else {
                }
            }
            None => {}
        }
    }

    fn split(&mut self, node: Box<Node>, u_box: QuadBox) {
        for n in node.children.iter() {}
    }
}

impl QuadTree {
    const MAX_DEPTH: u32 = 16;
    const MAX_NUM_OF_ELEMS: usize = 8;
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            root: Subtree::new(),
            u_box: QuadBox::new(0.0, 0.0, width, height),
            max_depth: Self::MAX_DEPTH,
            max_num_of_elems: Self::MAX_NUM_OF_ELEMS,
        }
    }
}
