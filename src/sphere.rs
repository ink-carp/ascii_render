use crate::{point::Point, FloatType, Vector};
use std::f64::consts::PI;
use std::io::Write;
pub struct Sphere {
    center: Point<FloatType>,
    radius: FloatType,
    resolution:usize,
}
impl Sphere {
    pub fn new(center: Point<FloatType>, radius:FloatType, resolution: usize) -> Self {
        Self {
            center,
            radius,
            resolution,
        }
    }
    pub fn generate_vertices(&self) -> Vec<Point<FloatType>> {
        let mut vertices = Vec::new();

        for i in 0..self.resolution {
            for j in 0..self.resolution {
                let theta = 2.0 * PI * i as FloatType / self.resolution as FloatType;
                let phi = PI * j as FloatType / self.resolution as FloatType;
                let x = self.center.get_x() + self.radius * theta.sin() * phi.sin();
                let y = self.center.get_y() + self.radius * phi.cos();
                let z = self.center.get_z() + self.radius * theta.cos() * phi.sin();
                vertices.push(Point::new(x, y, z));
            }
        }
        vertices
    }
    pub fn tomodel(self,path:&str){
        let vertices = self.generate_vertices();
        let mut normals:Vec<Vector> = Vec::with_capacity(vertices.len());
        for v in vertices.iter() {
            normals.push(v.sub(&self.center));
        }
        let mut file = std::fs::File::create(path).unwrap();
        for v in vertices.iter() {
            writeln!(file,"v {} {} {}",v.get_x(),v.get_y(),v.get_z()).unwrap();
        }
        for n in normals.iter() {
            writeln!(file,"vn {} {} {}",n.get_x(),n.get_y(),n.get_z()).unwrap();
        }
    }
}