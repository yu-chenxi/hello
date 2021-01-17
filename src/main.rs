use std::ops::AddAssign;

#[derive(Debug, Clone, Copy)]
struct Pt {
    x: i32,
    y: i32,
}

impl AddAssign for Pt {
    fn add_assign(&mut self, rhs: Self) {
        // todo!()
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Default for Pt {
    fn default() -> Self {
        // todo!()
        Pt { x: -1, y: -1 }
    }
}

fn main() {
    let mut pt1 = Pt { x: 1, y: 2 };
    let pt2 = Pt::default();
    pt1 += pt2;
    println!("pt1 = {:?}, pt2 = {:?}", pt1, pt2);
}
