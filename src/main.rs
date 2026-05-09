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

const IMAGE_WIDTH: f64 = 256.0;
const IMAGE_HEIGHT: f64 = 256.0;
const BLUE_TINT: f64 = 0.25;

fn main() {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT as i32).rev()  {
        for i in 0..IMAGE_WIDTH as i32 {
            let pixel = Vec3{
                x: i as f64/(IMAGE_WIDTH  - 1.0), // Red
                y: j as f64/(IMAGE_HEIGHT - 1.0), // Green
                z: BLUE_TINT // Blue
            };
            let ir = (255.999 * pixel.x) as i32; //  Varies Red as Δi
            let ig = (255.999 * pixel.y) as i32; // Varies Green as Δj
            let ib = (255.999 * pixel.z) as i32; //
            println!("{} {} {}", ir, ig, ib);
        }
        print!("\n");
    }
}
