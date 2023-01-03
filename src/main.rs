use glam::{uvec2, vec2, Vec3A};
use rayon::prelude::*;

use sdfu::SDF;
const MAX_MARCHES: u32 = 256;
const EPSILON: f32 = 0.0001;

fn main() {
    let width = 1280;
    let height = 800;
    //camera
    let sdf = sdfu::Sphere::new(1.0).translate(Vec3A::new(0.0, 0.0, 2.0));
    let aspect_ratio = width as f32 / height as f32;
    let eye = Vec3A::new(0.0, 0.0, -1.0);
    let up = Vec3A::new(0.0, -1.0, 0.0);
    let right = Vec3A::new(1.0 * aspect_ratio, 0.0, 0.0);
    let forward = up.cross(right).normalize();

    let pixels = (0..width * height)
        .into_par_iter()
        .map(|i| {
            let frag_coord = uvec2((i % width) as u32, (i / width) as u32);

            let uv = vec2(
                frag_coord.x as f32 / width as f32 * 2.0 - 1.0,
                frag_coord.y as f32 / height as f32 * 2.0 - 1.0,
            );

            // ray
            let origin = eye + forward * 1.0 + right * uv.x + up * uv.y;
            let direction = (origin - eye).normalize();
            let mut t = 0.0;

            let light_dir = Vec3A::new(-0.8, -1.0, 0.0).normalize();

            //colors
            let mut color = Vec3A::splat(0.0);
            let sky_color =
                Vec3A::new(1.0, 1.0, 1.0).lerp(Vec3A::new(0.6, 0.8, 0.9), uv.y * 0.5 + 1.2);

            // ray marching
            for _ in 0..MAX_MARCHES {
                let pos = origin + direction * t;
                let distance = sdf.dist(pos);

                if distance < EPSILON {
                    // Is there a way to implement a dynamic half_pixel_size instead of EPSILON?
                    let normals = sdf.normals_fast(EPSILON);
                    let normal = normals.normal_at(pos);
                    let dot = f32::powi(-light_dir.dot(normal) * 0.5 + 0.5, 2);

                    color = Vec3A::splat(dot);
                } else {
                    color = sky_color;
                }

                t += distance;
            }

            color
        })
        .collect::<Vec<Vec3A>>();

    let raw: Vec<u8> = pixels.iter().fold(Vec::new(), |mut v, c| {
        v.push((c.x * 255.0) as u8);
        v.push((c.y * 255.0) as u8);
        v.push((c.z * 255.0) as u8);
        v
    });

    image::save_buffer(
        "test.png",
        raw.as_slice(),
        width,
        height,
        image::ColorType::Rgb8,
    )
    .unwrap();

    println!("Finished");
}
