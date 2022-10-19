use core::f64::consts::PI;
use ray_tracer_challenge::*;

const SIZE: usize = 500;
const CAMERA_Z: f64 = -5.0;
const WALL_Z: f64 = 10.0;

fn main() {
    let mut world = World::default();
    world.light = Light::new_point(Point::new(-10, 10, -10), Color::white());

    world.add(
        Object::new(Sphere)
            .with_transform(Mat::identity().scale(0.75, 0.8, 1))
            .with_material(Material {
                color: Color::new(1, 0.2, 1),
                ambient: 0.2,
                ..Default::default()
            }),
    );
    world.add(
        Object::new(Sphere)
            .with_transform(Mat::identity().scale(0.5, 0.5, 0.5).translate(-0.75, 0, -1))
            .with_material(Material {
                color: Color::new(0.3, 0.8, 0.1),
                ambient: 0.2,
                ..Default::default()
            }),
    );
    world.add(
        Object::new(Sphere)
            .with_transform(
                Mat::identity()
                    .scale(0.5, 0.5, 0.5)
                    .translate(0.75, 0, -0.9),
            )
            .with_material(Material {
                color: Color::new(0.8, 0.8, 0.1),
                ambient: 0.2,
                ..Default::default()
            }),
    );

    let camera = Camera::new(
        SIZE,
        SIZE,
        PI / 3.0,
        Point::new(0, 0, CAMERA_Z),
        Point::new(0, 0, WALL_Z),
        Vector::new(0, 1, 0),
    );

    camera
        .render(&world)
        .write_ppm_file("/tmp/ch7.ppm")
        .expect("could not write PPM file");
}
