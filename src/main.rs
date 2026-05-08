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

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        let new_x = self.x - other.x;
        let new_y = self.y - other.y;
        let new_z = self.z - other.z;
        Vec3 {
            x: new_x,
            y: new_y,
            z: new_z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, scalar: f64 ) -> Vec3 {
        Vec3{
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, scalar: f64) -> Vec3 {
        Vec3{
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3{
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Vec3 {
    fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    fn  length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
}

fn unit_vector(v: Vec3) -> Vec3 {
    v/v.length()
}

fn main() {
    let pos1= Vec3{x:1.0, y:0.0, z:1.0};
    let pos2 = Vec3{x:2.0, y:3.0, z:4.0};
    println!("{:#?}", pos1 + pos2);
    println!("{:#?}", pos1 - pos2);
    println!("{:#?}", pos1 * pos2);
    println!("{:#?}", pos1 * 3.0);
    println!("{:#?}", pos1 / 3.0);
    println!("Hello, world!");
}
