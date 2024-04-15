use std::io::Write;

use crate::{FloatType, LightType, Point, Vector};


pub struct Model{
    vertices: Vec<Point<FloatType>>,//存储顶点信息
    normals: Vec<Vector>,//存储顶点法向量信息
    lightness: Vec<FloatType>,//存储顶点光照信息
}
impl Model{
    pub fn read(path:&str)->Self{
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let data = std::fs::read_to_string(path).unwrap();
        for line in data.lines(){
            let mut iter = line.split_whitespace();
            match iter.next(){
                Some("v")=>{
                    let x:FloatType = iter.next().unwrap().parse().unwrap();
                    let y:FloatType = iter.next().unwrap().parse().unwrap();
                    let z:FloatType = iter.next().unwrap().parse().unwrap();
                    vertices.push(Point::new(x,y,z));
                }
                Some("vn")=>{
                    let x:FloatType = iter.next().unwrap().parse().unwrap();
                    let y:FloatType = iter.next().unwrap().parse().unwrap();
                    let z:FloatType = iter.next().unwrap().parse().unwrap();
                    normals.push(Vector::new(x,y,z));
                }
                _=>{}
            }
        }
        Self{
            vertices,
            normals,
            lightness: Vec::new(),
        }
    }
    pub fn write(&self,path:&str){
        let mut file = std::fs::File::create(path).unwrap();
        for vertex in &self.vertices{
            writeln!(file,"v {} {} {}",vertex.get_x(),vertex.get_y(),vertex.get_z()).unwrap();
        }
        for normal in &self.normals{
            writeln!(file,"vn {} {} {}",normal.get_x(),normal.get_y(),normal.get_z()).unwrap();
        }
    }
    pub fn get_vertices(&self)->&Vec<Point<FloatType>>{
        &self.vertices
    }
    pub fn get_normals(&self)->&Vec<Vector>{
        &self.normals
    }
    pub fn get_lightness(&self)->&Vec<FloatType>{
        &self.lightness
    }
    pub fn add_light(&mut self,light:&LightType){
        self.lightness.clear();
        match light {
            LightType::Directional(directlight) => {
                for normal in &self.normals{
                    self.lightness.push(normal.normalize().dot(&directlight.get_direction().normalize()) * directlight.get_intensity());
                }
            },
            LightType::Point(_) => todo!(),
            LightType::Spot(_) => todo!(),
        }
    }
}