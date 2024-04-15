use crate::{Camera, FloatType, HomoCoord, LightType, Model, Plane};


pub struct World{
    width:FloatType,
    height:FloatType,
    deepth:FloatType,
    light:Option<LightType>,
    camera:Option<Camera>,
    model:Option<Model>
}
impl World {
    pub fn new(width:FloatType,height:FloatType,deepth:FloatType)->Self{
        Self{
            width,
            height,
            deepth,
            light:None,
            camera:None,
            model:None,
        }
    }
    pub fn add_light(&mut self,light:LightType){
        self.light = Some(light);
    }
    pub fn add_camera(&mut self,camera:Camera){
        self.camera = Some(camera);
    }
    pub fn add_model(&mut self,model:Model){
        self.model = Some(model);
    }
    pub fn get_width(&self)->FloatType{
        self.width
    }
    pub fn get_height(&self)->FloatType{
        self.height
    }
    fn shadow2byte(shadow:FloatType)->u8{
        //需要保证shadow的范围在0.0到1.0之间
        
        match (shadow * 255.) as u8{
            0 => b' ',
            1..=24 => b'.',
            25..=50 => b':',
            51..=76 => b'-',
            77..=124 => b'=',
            125..=148 => b'+',
            149..=172 => b'*',
            173..=194 => b'&',
            195..=216 => b'#',
            217..=u8::MAX => b'@',
        }
        // let brightness_order = " .:-=+*&#%@";
        // let index = (shadow * 10.0).floor() as usize;
        // brightness_order.chars().nth(index).unwrap() as u8
    }
    pub fn render(&mut self) -> Option<Plane>{
        if let Some(ref mut model) = self.model{
            if let Some(light) = &self.light{
                model.add_light(light);//添加光源
                if let Some(ref mut camera) = self.camera{
                    //投影到相机
                    let mut plane = Plane::new(camera.get_view_width().round() as usize, camera.get_view_height().round() as usize);
                    for (v,l) in model.get_vertices().iter().zip(model.get_lightness().iter()){
                        let world_coord = HomoCoord::from_point(v);  
                        //将中心移至世界中心
                        let world_coord =world_coord.add(&HomoCoord::new(self.width/2., self.height/2., self.deepth/2., 0.));
                        //println!("{} {} {} {}",world_coord.get_x(),world_coord.get_y(),world_coord.get_z(),world_coord.get_w());
                        if let Some(screen_coord) = camera.project(world_coord){
                            let real_x = ((screen_coord.get_x() + 1.0) / 2.0 * camera.get_view_width()).round() as usize;
                            let real_y = ((screen_coord.get_y() + 1.0) / 2.0 * camera.get_view_height()).round() as usize;
                            if screen_coord.get_z() < plane.get(real_x, real_y).get_deepth(){
                                plane.get_mut(real_x, real_y).set_byte(Self::shadow2byte(*l));
                                plane.get_mut(real_x, real_y).set_deepth(screen_coord.get_z());
                            }
                        }
                    }
                    return Some(plane);
                }
            }
        }
        None
    }
}