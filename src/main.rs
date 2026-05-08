use std::ops;

#[derive(Clone, Copy, Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        let new_x = self.x + other.x;
        let new_y = self.y + other.y;
        let new_z = self.z + other.z;
        Vec3 {
            x: new_x,
            y: new_y,
            z: new_z,
        }
    }
}

fn main() {

    let pos1= Vec3{x:1.0, y:0.0, z:1.0};
    let post2 = Vec3{x:2.0, y:3.0, z:4.0};
    println!("{:#?}", pos1 + post2);
    println!("Hello, world!");
}
