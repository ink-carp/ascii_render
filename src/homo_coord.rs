use num_traits::{Num,  NumCast, FromPrimitive, ToPrimitive};

use crate::{Point, Vector};
pub struct HomoCoord<T>
where T: Num + Copy + ToPrimitive + FromPrimitive + Default + Clone + NumCast
{
    x: T,
    y: T,
    z: T,
    w: T,
}
impl<T> HomoCoord<T>
where T: Num + Copy + ToPrimitive + FromPrimitive + Default + Clone + NumCast
{
    pub fn new(x: T, y: T, z: T, w: T) -> HomoCoord<T> {
        HomoCoord { x, y, z, w }
    }
    pub fn get_x(&self) -> T {
        self.x
    }
    pub fn get_y(&self) -> T {
        self.y
    }
    pub fn get_z(&self) -> T {
        self.z
    }
    pub fn get_w(&self) -> T {
        self.w
    }
    pub fn set_x(&mut self, x: T) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: T) {
        self.y = y;
    }
    pub fn set_z(&mut self, z: T) {
        self.z = z;
    }
    pub fn set_w(&mut self, w: T) {
        self.w = w;
    }
    pub fn from_point(point: &Point<T>) -> HomoCoord<T> {
        HomoCoord {
            x: point.get_x(),
            y: point.get_y(),
            z: point.get_z(),
            w: T::one(),
        }
    }
    pub fn from_cevtor(vector: &Vector) -> HomoCoord<T> {
        HomoCoord {
            x: NumCast::from(vector.get_x()).unwrap(),
            y: NumCast::from(vector.get_y()).unwrap(),
            z: NumCast::from(vector.get_z()).unwrap(),
            w: T::zero(),
        }
    }
    // 将一个HomoCoord实例转换为一个Point实例
    pub fn to_point(&self) -> Point<T> {
        Point::new(
            self.x / self.w,
            self.y / self.w,
            self.z / self.w,
        )
    }

    // 将这个HomoCoord实例进行缩放
    pub fn scale(&mut self, factor: T) {
        self.x = self.x * factor;
        self.y = self.y * factor;
        self.z = self.z * factor;
        self.w = self.w * factor;
    }

    // 将这个HomoCoord实例进行归一化，即除以w
    pub fn normalize(&mut self) {
        let w = self.w;
        self.x = self.x / w;
        self.y = self.y / w;
        self.z = self.z / w;
        self.w = T::one();
    }
    // 计算这个HomoCoord实例与另一个HomoCoord实例的点积
    pub fn dot(&self, other: &HomoCoord<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    // 计算这个HomoCoord实例与另一个HomoCoord实例的叉积
    pub fn cross(&self, other: &HomoCoord<T>) -> HomoCoord<T> {
        HomoCoord {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: T::one(),
        }
    }

    // 将这个HomoCoord实例与另一个HomoCoord实例相加
    pub fn add(&self, other: &HomoCoord<T>) -> HomoCoord<T> {
        HomoCoord {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }

    // 将这个HomoCoord实例与另一个HomoCoord实例相减
    pub fn sub(&self, other: &HomoCoord<T>) -> HomoCoord<T> {
        HomoCoord {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }

    // 将这个HomoCoord实例与另一个HomoCoord实例或一个标量相乘
    pub fn mul(&self, factor: T) -> HomoCoord<T> {
        HomoCoord {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
            w: self.w * factor,
        }
    }
    pub fn div(&self, factor: T) -> HomoCoord<T> {
        HomoCoord {
            x: self.x / factor,
            y: self.y / factor,
            z: self.z / factor,
            w: self.w / factor,
        }
    }
}