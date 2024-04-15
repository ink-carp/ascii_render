use num_traits::{Num,  NumCast, FromPrimitive, ToPrimitive};

use crate::{Vector,FloatType};
#[derive(Default, Clone, Copy,Debug)]
pub struct Point<T>
where T: Num + Copy + ToPrimitive + FromPrimitive
{
    x: T,
    y: T,
    z: T,
}
impl<T> Point<T>
where T:Num + Copy + ToPrimitive + FromPrimitive
{
    pub fn new(x: T, y: T, z: T) -> Point<T> {
        Point { x, y, z }
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
    pub fn set_x(&mut self, x: T) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: T) {
        self.y = y;
    }
    pub fn set_z(&mut self, z: T) {
        self.z = z;
    }
    pub fn distance(&self, other: &Point<T>) -> FloatType {
        let x:FloatType = NumCast::from(self.x - other.get_x()).unwrap();
        let y:FloatType = NumCast::from(self.y - other.get_y()).unwrap();
        let z:FloatType = NumCast::from(self.z - other.get_z()).unwrap();
        let x = x * x;
        let y = y * y;
        let z = z * z;
        let distance = (x + y + z).sqrt();
        distance
    }
    pub fn add_other(&self,direction:Vector)->Point<FloatType>{
        Point::new( <FloatType as NumCast>::from(self.x).unwrap()+direction.get_x(),
        <FloatType as NumCast>::from(self.y).unwrap()+direction.get_y(),
        <FloatType as NumCast>::from(self.z).unwrap()+direction.get_z())
    }
    pub fn sub_other(&self,direction:Vector)->Point<FloatType>{
        Point::new( <FloatType as NumCast>::from(self.x).unwrap()-direction.get_x(),
        <FloatType as NumCast>::from(self.y).unwrap()-direction.get_y(),
        <FloatType as NumCast>::from(self.z).unwrap()-direction.get_z())
    }
    pub fn to_vector(&self)->Vector{
        Vector::new(
            <FloatType as NumCast>::from(self.x).unwrap(),
            <FloatType as NumCast>::from(self.y).unwrap(),
            <FloatType as NumCast>::from(self.z).unwrap()
        )
    }
}
impl Point<FloatType> {
    pub fn sub(&self,other:&Point<FloatType>)->Vector{
        Vector::new(
            self.x-other.get_x(),
            self.y-other.get_y(),
            self.z-other.get_z()
        )
    }
}