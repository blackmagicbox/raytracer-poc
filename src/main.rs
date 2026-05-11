use std::{env::VarError, ops};

const IMAGE_WIDTH: f64 = 256.0;
const IMAGE_HEIGHT: f64 = 256.0;
const BLUE_TINT: f64 = 0.25;
const ASPECT_RATIO: f64 = IMAGE_WIDTH / IMAGE_HEIGHT;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }
    fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

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
    fn mul(self, scalar: f64) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, scalar: f64) -> Vec3 {
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Vec3 {
    fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
}

fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

fn ray_color(r: &Ray) -> Vec3 {
    let unit_direction = unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3 {
        x: (1.0 - t) * 1.0 + t * 0.5, // Red
        y: (1.0 - t) * 1.0 + t * 0.7, // Green
        z: (1.0 - t) * 1.0 + t * 1.0, // Blue
    }
}

fn main() {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    let camera_origin = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let horizontal = Vec3 {
        x: VIEWPORT_WIDTH,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: VIEWPORT_HEIGHT,
        z: 0.0,
    };
    let lower_left_corner = camera_origin
        - horizontal / 2.0
        - vertical / 2.0
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: FOCAL_LENGTH,
        };

    for j in (0..IMAGE_HEIGHT as i32).rev() {
        for i in 0..IMAGE_WIDTH as i32 {
            let u = i as f64 / (IMAGE_WIDTH - 1.0);
            let v = j as f64 / (IMAGE_HEIGHT - 1.0);
            let ray_direction = lower_left_corner + horizontal * u + vertical * v - camera_origin;
            let r = Ray::new(camera_origin, ray_direction);
            let pixel_color = ray_color(&r);
            let ir = (255.999 * pixel_color.x) as i32;
            let ig = (255.999 * pixel_color.y) as i32;
            let ib = (255.999 * pixel_color.z) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
