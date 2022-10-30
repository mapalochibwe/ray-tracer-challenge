use image::RgbImage;
use ray_tracer_challenge::*;

const SIZE: u32 = 1000;
const CAMERA_Z: f64 = -5.0;
const WALL_Z: f64 = 10.0;
const WALL_SIZE: f64 = 7.0;

fn main() {
    let mut img = RgbImage::new(SIZE, SIZE);

    let s = Object::new(Sphere)
        .with_transform(Mat::identity().scale(0.75, 0.8, 1))
        .with_material(Material {
            pattern: Color::new(1, 0.2, 1).into(),
            ..Default::default()
        });
    let origin = Point::new(0, 0, CAMERA_Z);

    let light = Light::new_point(Point::new(-10, 10, -10), Color::white());

    for x in 0..SIZE {
        let wall_x = (x as f64 / SIZE as f64) * WALL_SIZE - WALL_SIZE / 2.0;
        for y in 0..SIZE {
            let wall_y = (y as f64 / SIZE as f64) * WALL_SIZE - WALL_SIZE / 2.0;
            let wall_pt = Point::new(wall_x, -wall_y, WALL_Z);
            let ray = Ray::new(origin, (wall_pt - origin).normalize());

            let mut inters = Intersections::default();
            s.intersect(ObjectIndex::test_value(1), &ray, &mut inters);
            if let Some(inter) = inters.hit() {
                let hit_pt = ray.position(inter.t);
                let (normal, _) = s.normal_and_color(hit_pt);
                let eye = -ray.direction;
                img.put_pixel(
                    x,
                    y,
                    light
                        .lighting(Color::white(), &s.material, hit_pt, eye, normal, false)
                        .into(),
                );
            } else {
                img.put_pixel(x, y, Color::new(0, 0, 1).into());
            }
        }
    }

    img.save("/tmp/ch6.png").expect("could not write PNG file");
}
