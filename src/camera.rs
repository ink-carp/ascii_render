use crate::{point::Point, vector::Vector, FloatType, HomoCoord, Matrix4};
/// 表示一个3D场景中的相机。
#[derive(Debug)]
pub struct Camera {
    //需要设置的
    position: Point<FloatType>,   // 相机在3D空间中的位置。
    senser_width: FloatType,      // 感光元件的宽度。单位为mm
    direction: Vector,      // 相机的朝向。
    focal_length: FloatType,      // 相机的焦距。单位为mm
    near_clip: FloatType,   // 相机的近裁剪面。
    view_port_distance: FloatType, // 相机视口的距离。
    far_clip: FloatType,    // 相机的远裁剪面。
    ratio: FloatType,       // 相机视口的宽高比。width/height

    //自动计算的
    view_center: Point<FloatType>, // 相机视口的中心。
    up: Vector,             // 相机的上方向向量。
    right: Vector,          // 相机的右方向向量。
    view_width: FloatType,             // 相机视口的宽度。
    view_height: FloatType,            // 相机视口的高度。
    fov_x: FloatType,             // 相机的水平视场角。单位为弧度
    fov_y: FloatType,             // 相机的垂直视场角。单位为弧度
    //plane:Plane,
}
impl Camera{
    pub fn new(position:Point<FloatType>)->Self{
        Self{
            position,
            senser_width: 0.,
            direction: Vector::default(),
            up: Vector::default(),
            right: Vector::default(),
            view_width: 0.,
            view_height: 0.,
            focal_length: 0.,
            view_port_distance: 0.,
            fov_x: 0.,
            fov_y: 0.,
            near_clip: 0.,
            far_clip: 0.,
            ratio: 1.,
            view_center: Point::default(),
            //plane:Plane::new(0,0),
        }
    }
    pub fn get_view_width(&self)->FloatType{
        self.view_width
    }
    pub fn get_view_height(&self)->FloatType{
        self.view_height
    }
    pub fn set_view_port_distance(&mut self,distance:FloatType){
        self.view_port_distance = distance;
    }
    // pub fn get_plane(&self)->&Plane{
    //     &self.plane
    // }
    // pub fn get_plane_mut(&mut self)->&mut Plane{
    //     &mut self.plane
    // }
    pub fn set_senser_width(&mut self,width:FloatType){
        self.senser_width = width;
    }
    pub fn set_position(&mut self,position:Point<FloatType>){
        self.position = position;
        //self.view_center = self.position.add_other(self.direction.multiply_scalar(self.near_clip));
    }
    //需要传入一个归一化后的方向向量
    pub fn set_direction(&mut self,direction:Vector){
        self.direction = direction;
    }
    pub fn set_facal(&mut self,focal:FloatType){
        self.focal_length = focal;
    }
    pub fn set_clip(&mut self,near_clip:FloatType,far_clip:FloatType){
        self.near_clip = near_clip;
        self.far_clip = far_clip;
    }
    pub fn set_ratio(&mut self,ratio:FloatType){
        self.ratio = ratio;
    }
    pub fn update_camera(&mut self){
        let z_axis = self.direction;
        let temp =  if z_axis.get_x() == 0. && z_axis.get_y() == 0. {
            Vector::new(1., 0., 0.)
        }else{
            Vector::new(0., 0., 1.)
        };
        let x_axis = z_axis.cross(&temp).normalize();
        let y_axis = z_axis.cross(&x_axis).normalize();
        self.fov_x = 2.0 * (self.senser_width/(2.0*self.focal_length)).atan();
        self.fov_y = 2.0 * ((self.fov_x/2.0).tan() * self.ratio).atan();
        self.view_height = 2.0 * (self.fov_y/2.0).tan() * self.view_port_distance;
        self.view_width = self.ratio * self.view_height;
        self.right = x_axis;
        self.up = y_axis;
        self.view_center = self.position.add_other(self.direction.multiply_scalar(self.view_port_distance));
        // println!("{} {} {}",z_axis.get_x(),z_axis.get_y(),z_axis.get_z());
        // println!("{} {} {}",x_axis.get_x(),x_axis.get_y(),x_axis.get_z());
        // println!("{} {} {}",y_axis.get_x(),y_axis.get_y(),y_axis.get_z());
    }
    pub fn get_view_plane_corners(&mut self) -> [Point<FloatType>; 4] {
        let right = self.right.multiply_scalar(self.view_width/2.);
        let up = self.up.multiply_scalar(self.view_height/2.);
        [
            self.view_center.sub_other(right).sub_other(up),
            self.view_center.add_other(right).sub_other(up),
            self.view_center.add_other(right).add_other(up),
            self.view_center.sub_other(right).add_other(up),
        ]
    }
    fn get_view_matrix(&self)->Matrix4<FloatType>{
        let r = self.right;
        let u = self.up;
        let d = self.direction;
        let t = self.position.to_vector();
        //println!("{:#?}",t);
        Matrix4::from_vec(
            vec![
                r.get_x(), u.get_x(), d.get_x(), t.get_x(),
                r.get_y(), u.get_y(), d.get_y(), t.get_y(),
                r.get_z(), u.get_z(), d.get_z(), t.get_z(),
                0.0, 0.0, 0.0, 1.0,
            ]
        )
    }
    fn get_projection_matrix(&self)->Matrix4<FloatType>{
        let aspect = self.ratio;
        let fov_y = self.fov_y;
        let near = self.near_clip;
        let far = self.far_clip;
        let f = 1.0/(fov_y/2.0).tan();
        Matrix4::from_vec(
            vec![
                f/aspect, 0.0, 0.0, 0.0,
                0.0, f, 0.0, 0.0,
                0.0, 0.0, (far+near)/(near-far), 2.0*far*near/(near-far),
                0.0, 0.0, -1.0, 0.0,
            ]
        )
    }
    pub fn project(&self,coord:HomoCoord<FloatType>)->Option<HomoCoord<FloatType>>{
        let view_matrix = self.get_view_matrix();
        let camera_coord = view_matrix.mul_homo(coord);
        //println!("{} {} {} {}",camera_coord.get_x(),camera_coord.get_y(),camera_coord.get_z(),camera_coord.get_w());
        let projection_coord = self.get_projection_matrix().mul_homo(camera_coord);
        //println!("{} {} {} {}",projection_coord.get_x(),projection_coord.get_y(),projection_coord.get_z(),projection_coord.get_w());
        let screen_coord = projection_coord.div(projection_coord.get_w());
        //println!("{} {} {} {}",screen_coord.get_x(),screen_coord.get_y(),screen_coord.get_z(),screen_coord.get_w());
        if screen_coord.get_x() < -1. || screen_coord.get_x() > 1. || screen_coord.get_y() < -1. || screen_coord.get_y() > 1.{
            None
        }else {
            Some(screen_coord)
        }
    }
}