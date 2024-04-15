use num_traits::{FromPrimitive, Num, ToPrimitive};

use crate::{FloatType, HomoCoord, Point};
pub struct Matrix<T>
where T: Default + Clone + Copy
{
    data: Vec<Vec<T>>,
    row: usize,
    col: usize,
}
impl<T> Matrix<T>
where T: Default + Clone + Copy
{
    pub fn new(row:usize,col:usize)->Self{
        let mut data = Vec::with_capacity(row);
        for _ in 0..row{
            let mut row = Vec::with_capacity(col);
            for _ in 0..col{
                row.push(T::default());
            }
            data.push(row);
        }
        Self{
            data,
            row,
            col,
        }
    }
    pub fn get(&self,row:usize,col:usize)->&T{
        &self.data[row][col]
    }
    pub fn get_mut(&mut self,row:usize,col:usize)->&mut T{
        &mut self.data[row][col]
    }
    pub fn set(&mut self,row:usize,col:usize,cell:T){
        self.data[row][col] = cell;
    }
    pub fn get_row(&self)->usize{
        self.row
    }
    pub fn get_col(&self)->usize{
        self.col
    }
    pub fn reverse_rows(&mut self) {
        self.data.reverse();
    }
}
#[derive(Default)]
pub struct Matrix4<T>
where T: Default + Clone + Copy + Num + ToPrimitive + FromPrimitive
{
    data: Vec<T>,
}
impl<T> Matrix4<T>
where T: Default + Clone + Copy + Num + ToPrimitive + FromPrimitive
{
    // 创建一个新的单位矩阵
    pub fn identity() -> Self {
        let mut data = vec![T::zero(); 16];
        data[0] = T::one();
        data[5] = T::one();
        data[10] = T::one();
        data[15] = T::one();
        Self { data }
    }
    pub fn zero() -> Self {
        let data = vec![T::zero(); 16];
        Self { data }
    }
    pub fn set(&mut self, row: usize, col: usize, cell: T) {
        self.data[row * 4 + col] = cell;
    }

    // 将这个矩阵进行缩放
    // 大于 1 放大
    //小于 1 缩小
    pub fn scale(&mut self, factor: T) {
        for i in 0..16 {
            self.data[i] = self.data[i] * factor;
        }
    }

    // 将这个矩阵与另一个矩阵相加
    pub fn add(&mut self, other: &Self) {
        for i in 0..16 {
            self.data[i] = self.data[i] + other.data[i];
        }
    }

    // 将这个矩阵与另一个矩阵相减
    pub fn sub(&mut self, other: &Self) {
        for i in 0..16 {
            self.data[i] = self.data[i] - other.data[i];
        }
    }
    pub fn to_point(self)->Point<T>{
        Point::new(self.data[0]/self.data[3], self.data[1]/self.data[3], self.data[2]/self.data[3])
    }
}
impl Matrix4<FloatType> {
    pub fn mul_homo(&self, home_coord: HomoCoord<FloatType>) -> HomoCoord<FloatType> {
        let mut newx = 0.0;
        let mut newy = 0.0;
        let mut newz = 0.0;
        let mut neww = 0.0;
        
        newx += self.data[0] * home_coord.get_x() + self.data[1] * home_coord.get_y() + self.data[2] * home_coord.get_z() + self.data[3] * home_coord.get_w();
        newy += self.data[4] * home_coord.get_x() + self.data[5] * home_coord.get_y() + self.data[6] * home_coord.get_z() + self.data[7] * home_coord.get_w();
        newz += self.data[8] * home_coord.get_x() + self.data[9] * home_coord.get_y() + self.data[10] * home_coord.get_z() + self.data[11] * home_coord.get_w();
        neww += self.data[12] * home_coord.get_x() + self.data[13] * home_coord.get_y() + self.data[14] * home_coord.get_z() + self.data[15] * home_coord.get_w();
    
        HomoCoord::new(newx, newy, newz, neww)
    }
    pub fn from_vec(data:Vec<FloatType>)->Self{
        Self{
            data
        }
    }
}