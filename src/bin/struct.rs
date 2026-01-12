#![allow(unused)]

// Struct

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

struct Point3d(f32, f32, f32); // puede declararse en "formato tupla"

struct Empty;

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: f32,
}

fn main() {
    // Create
    let p = Point { x: 1.0, y: 2.0 };
    println!("point.x = {}, point.y = {}", p.x, p.y);

    let circle = Circle {
        center: p,
        radius: 1.0,
    };

    // Debug
    // Read
    println!("{:?}", circle);
    // Shortcut
    let x = 1.0;
    let y = 1.0;
    let p = Point { x, y };
    // Copy fields
    let p0 = Point { x: 1.0, y: 1.0 };
    let p1 = Point { x: 2.0, y: p0.y }; // también { x: 2.0, ..p0 };

    // Update
    let mut p = Point { x: 0.0, y: 0.0 };
    p.x += 1.1;
    p.y += 1.0;
}
