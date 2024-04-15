use std::fmt::Display;

use crate::{Matrix, Pixel};
pub struct Plane{
    data:Matrix<Pixel>,
}
impl Plane {
    pub fn new(row:usize,col:usize)->Self{
        let data = Matrix::new(row,col);
        Self{
            data,
        }
    }
    pub fn get(&self,row:usize,col:usize)->&Pixel{
        self.data.get(row,col)
    }
    pub fn get_mut(&mut self,row:usize,col:usize)->&mut Pixel{
        self.data.get_mut(row, col)
    }
    pub fn set(&mut self,row:usize,col:usize,pixel:Pixel){
        self.data.set(row,col,pixel);
    }
}
impl Display for Plane{
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
        for row in 0..self.data.get_row(){
            for col in 0..self.data.get_col(){
                write!(f,"{}",self.data.get(row,col).get_byte() as char)?;
            }
            write!(f,"\n")?;
        }
        Ok(())
    }
}