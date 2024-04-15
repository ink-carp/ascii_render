use ascii_render::*;
fn main() {
    //创建一个球形模型
    // let center = Point::new(0.0, 0.0, 0.0);
    // let radius = 30.0;
    // let resolution = 200;
    // let sphere = Sphere::new(center, radius, resolution);
    // sphere.tomodel("sphere.obj");


    // let mut model = Model::read("sphere.obj");
    // let light = LightType::Directional(DirectionalLight::new(Vector::new(0., 0., -1.), 1.0));
    // model.add_light(&light);
    // let mut plane = Plane::new(100, 100);
    // for (v,l) in model.get_vertices().iter().zip(model.get_lightness().iter()) {
    //     let x = (v.get_x() + 50.).round() as usize;
    //     let y = (v.get_y() + 50.).round() as usize;
    //     let z = v.get_z() + 50.;
    //     if z < plane.get(x, y).get_deepth(){
    //         //println!("{} {} {}",x,y,z);
    //         plane.get_mut(x, y).set_byte(shadow2byte(*l));
    //         plane.get_mut(x, y).set_deepth(z);
    //     }
    //     // println!("{} {} {}",x,y,z);
    // }
    // println!("{}",plane);


    let model = Model::read("sphere.obj");
    let light = LightType::Directional(DirectionalLight::new(Vector::new(0., 0., -1.), 1.0));
    let mut camera = Camera::new(Point::new(50., 50., 100.));
    camera.set_direction(Vector::new(0., 0., -1.));
    camera.set_clip(1., 100.);
    camera.set_facal(0.1);
    camera.set_senser_width(0.2);
    camera.set_view_port_distance(30.);
    camera.set_ratio(1.);
    camera.update_camera();
    //println!("{:#?}",camera);
    //println!("{} {}",camera.get_width(),camera.get_height());
    let mut world = World::new(100., 100.0, 100.0);
    world.add_camera(camera);
    world.add_light(light);
    world.add_model(model);
    let plane = world.render().unwrap();
    println!("{}", plane);
}

// fn shadow2byte(shadow:FloatType)->u8{
//     //需要保证shadow的范围在0.0到1.0之间
    
//     match (shadow * 255.) as u8{
//         0 => b' ',
//         1..=24 => b'.',
//         25..=50 => b':',
//         51..=76 => b'-',
//         77..=124 => b'=',
//         125..=148 => b'+',
//         149..=172 => b'*',
//         173..=194 => b'&',
//         195..=216 => b'#',
//         217..=u8::MAX => b'@',
//     }
//     // let brightness_order = " .:-=+*&#%@";
//     // let index = (shadow * 10.0).floor() as usize;
//     // brightness_order.chars().nth(index).unwrap() as u8
// }