#[allow(clippy::float_cmp)]
extern crate rand;
mod vec3;
mod point;
mod utility;
mod ray;
mod sphere;
mod camera;
mod hit;
mod material;
use image::{ImageBuffer, RgbImage};
use material::*;
pub use vec3::Vec3;
pub use ray::Ray;
pub use hit::*;
pub use utility::*;
pub use sphere::Sphere;
pub use camera::Camera;
use indicatif::ProgressBar;
use std::sync::Arc;
fn main(){
    // Image
    const aspect_ratio :f64 = 16.0 / 9.0;
    const image_width :u32 = 400;
    const image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    const samples_per_pixel :u32 = 100;
    const max_depth:i32 = 50;
    // World
    let R = (std::f64::consts::PI/4.0).cos();
    let mut world:HittableList = HittableList::new();
    let  material_left   = Arc::new(Lambertian{
        albedo: Vec3::new(0.0,0.0,1.0),
    });
    let  material_right  = Arc::new(Lambertian{
        albedo: Vec3::new(1.0,0.0,0.0),
    });
    world.objects.push( Box::new( Sphere::new(Vec3::new( -R, 0.0, -1.0), R, material_left)));
    world.objects.push( Box::new( Sphere::new(Vec3::new( R, 0.0, -1.0), R, material_right)));

    // Camera
    let cam: Camera = Camera::new(90.0,aspect_ratio);
    let bar = ProgressBar::new(image_height as u64);

    // Render
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    for j in 0..image_height {
        for i in 0..image_width {
            let mut pixel_color= Vec3::zero();
            for s in 0..samples_per_pixel {//随机采样
                let u = (i as f64+ rand::random::<f64>())/(image_width as f64 - 1.0);
                let v = (j as f64+ rand::random::<f64>())/(image_height as f64 - 1.0);
                let ray :Ray = cam.get_ray(u, v);
                pixel_color += ray_color3(&ray, &world,max_depth);
            }

            let pixel: &mut image::Rgb<u8> = img.get_pixel_mut(i, j);
            grey_color(pixel, &pixel_color,samples_per_pixel);
        }
        bar.inc(1);
    }
    img.save("output/camera.png").unwrap();
    bar.finish();
}
fn main10() {
    // Image
    const aspect_ratio :f64 = 16.0 / 9.0;
    const image_width :u32 = 400;
    const image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    const samples_per_pixel :u32 = 100;
    const max_depth:i32 = 50;
    // World
    let mut world:HittableList = HittableList::new();

    let  material_ground = Arc::new(Lambertian{
        albedo: Vec3::new(0.8, 0.8, 0.0)
    });
    let  material_center = Arc::new(Lambertian{
        albedo: Vec3::new(0.1, 0.2, 0.5)
    });
    let  material_left   = Arc::new(Dielectric{
        ref_idx : 1.5,
    });
    let  material_right  = Arc::new(Metal{
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz : 0.0,
    });

    world.objects.push( Box::new( Sphere::new(Vec3::new( 0.0, -100.5, -1.0), 100.0, material_ground)));
    world.objects.push( Box::new( Sphere::new(Vec3::new( 0.0, 0.0, -1.0), 0.5, material_center)));
    world.objects.push( Box::new( Sphere::new(Vec3::new( -1.0, 0.0, -1.0), 0.5, material_left.clone())));
    world.objects.push( Box::new( Sphere::new(Vec3::new( -1.0, 0.0, -1.0), -0.4, material_left)));
    world.objects.push( Box::new( Sphere::new(Vec3::new( 1.0, 0.0, -1.0), 0.5, material_right)));
    
    // Bar
    let bar = ProgressBar::new(image_height as u64);

    // Camera
    let cam :Camera = Camera::standard() ;

    // Render
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    for j in 0..image_height {
        for i in 0..image_width {
            let mut pixel_color= Vec3::zero();
            for s in 0..samples_per_pixel {//随机采样
                let u = (i as f64+ rand::random::<f64>())/(image_width as f64 - 1.0);
                let v = (j as f64+ rand::random::<f64>())/(image_height as f64 - 1.0);
                let ray :Ray = cam.get_ray(u, v);
                pixel_color += ray_color3(&ray, &world,max_depth);
            }

            let pixel: &mut image::Rgb<u8> = img.get_pixel_mut(i, j);
            grey_color(pixel, &pixel_color,samples_per_pixel);
        }
        bar.inc(1);
    }
    img.save("output/camera.png").unwrap();
    bar.finish();
}

// fn ray_color1(ray:&Ray,world:&dyn Hittable) -> Vec3 {
//     let mut rec= HitRecord::new();
//     if world.hit(&ray,0.001,std::f64::INFINITY,& mut rec){
//         return (rec.normal+Vec3::new(1.0, 1.0, 1.0))*0.5;
//     }
//     else {
//         let unit_direction: Vec3 = ray.dir.unit();
//         let t = 0.5*(unit_direction.y+1.0);
//         return Vec3::ones()*(1.0-t)+Vec3::new(0.5,0.7,1.0)*t;
//     }
// }

fn ray_color2(ray:&Ray,world:&dyn Hittable,depth: i32) -> Vec3 {
    if depth <= 0{
        return Vec3::zero();
    }

    let mut rec:HitRecord = HitRecord::new(Arc::new(Lambertian{
        albedo: Vec3::zero(),
    }));

    if world.hit(&ray,0.001,std::f64::INFINITY,& mut rec){
        let target = rec.p +rec.normal +Vec3::random_unit_vector();
        return ray_color2(&Ray::new(rec.p,target-rec.p), world, depth-1)*0.5;
    }
    else {
        let unit_direction: Vec3 = ray.dir.unit();
        let t = 0.5*(unit_direction.y+1.0);
        return Vec3::ones()*(1.0-t)+Vec3::new(0.5,0.7,1.0)*t;
    }
}

fn ray_color3(ray:&Ray,world:&dyn Hittable,depth: i32) -> Vec3 {
    if depth <= 0{
        return Vec3::zero();
    }

    let mut rec:HitRecord = HitRecord::new(Arc::new(Lambertian{
        albedo: Vec3::zero(),
    }));

    if world.hit(&ray,0.001,std::f64::INFINITY,& mut rec){
        let mut scattered: Ray = Ray::new(Vec3::zero(), Vec3::zero());
        let mut attenuation: Vec3 = Vec3::zero();
        if rec.mat_ptr.scatter(ray, &rec, &mut attenuation, &mut scattered) {
            return Vec3::elemul(attenuation,ray_color3(&scattered, world, depth-1));
        }
        else {
            return Vec3::zero();
        }
    }
    else {
        let unit_direction: Vec3 = ray.dir.unit();
        let t = 0.5*(unit_direction.y+1.0);
        return Vec3::ones()*(1.0-t)+Vec3::new(0.5,0.7,1.0)*t;
    }
}

fn white_color(pixel: &mut image::Rgb<u8>, pixel_color :&Vec3,samples_per_pixel: u32){
    let r = pixel_color.x/samples_per_pixel as f64;
    let g = pixel_color.y/samples_per_pixel as f64;
    let b = pixel_color.z/samples_per_pixel as f64;

    *pixel = image::Rgb([
        (256.0 *clamp(r, 0.0, 0.999)) as u8 ,
        (256.0 *clamp(g, 0.0, 0.999)) as u8 ,
        (256.0 *clamp(b, 0.0, 0.999)) as u8 
    ]);
}
fn grey_color(pixel: &mut image::Rgb<u8>, pixel_color :&Vec3,samples_per_pixel: u32){
    let r = (pixel_color.x/samples_per_pixel as f64).sqrt();
    let g = (pixel_color.y/samples_per_pixel as f64).sqrt();
    let b = (pixel_color.z/samples_per_pixel as f64).sqrt();

    *pixel = image::Rgb([
        (256.0 *clamp(r, 0.0, 0.999)) as u8 ,
        (256.0 *clamp(g, 0.0, 0.999)) as u8 ,
        (256.0 *clamp(b, 0.0, 0.999)) as u8 
    ]);
}