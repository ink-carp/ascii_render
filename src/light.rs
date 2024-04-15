use crate::{FloatType, Vector};
// 定义光源类型的枚举
pub enum LightType {
    Directional(DirectionalLight),
    Point(PointLight),
    Spot(SpotLight),
}

// 定义定向光源的结构体
pub struct DirectionalLight {
    direction: Vector, // 光源方向
    intensity: FloatType, // 光源强度
}

// 定义点光源的结构体
pub struct PointLight {
    position: Vector, // 光源位置
    intensity: FloatType, // 光源强度
}

// 定义聚光灯的结构体
pub struct SpotLight {
    position: Vector, // 光源位置
    direction: Vector, // 光源方向
    intensity: FloatType, // 光源强度
    angle: FloatType, // 聚光灯的开角
}
impl DirectionalLight {
    pub fn new(direction: Vector, intensity: FloatType) -> Self {
        Self { direction, intensity }
    }
    pub fn get_direction(&self) -> &Vector {
        &self.direction
    }
    pub fn get_intensity(&self) -> FloatType {
        self.intensity
    }
    pub fn set_direction(&mut self, direction: Vector) {
        self.direction = direction;
    }
    pub fn set_intensity(&mut self, intensity: FloatType) {
        self.intensity = intensity;
    }
}

impl PointLight {
    pub fn new(position: Vector, intensity: FloatType) -> Self {
        Self { position, intensity }
    }
    pub fn get_position(&self) -> &Vector {
        &self.position
    }
    pub fn get_intensity(&self) -> FloatType {
        self.intensity
    }
}

impl SpotLight {
    pub fn new(position: Vector, direction: Vector, intensity: FloatType, angle: FloatType) -> Self {
        Self { position, direction, intensity, angle }
    }
    pub fn get_position(&self) -> &Vector {
        &self.position
    }
    pub fn get_direction(&self) -> &Vector {
        &self.direction
    }
    pub fn get_intensity(&self) -> FloatType {
        self.intensity
    }
    pub fn get_angle(&self) -> FloatType {
        self.angle
    }
}
