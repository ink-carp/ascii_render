pub type FloatType = f64;
#[derive(Default, Clone, Copy,Debug)]
pub struct Vector{
    vec_x: FloatType,
    vec_y: FloatType,
    vec_z: FloatType,
}

impl Vector
{
    pub fn new(vec_x:FloatType , vec_y:FloatType , vec_z:FloatType ) -> Vector {
        Vector { vec_x, vec_y,vec_z }
    }
    pub fn get_x(&self) -> FloatType {
        self.vec_x
    }
    pub fn get_y(&self) -> FloatType {
        self.vec_y
    }
    pub fn get_z(&self) -> FloatType {
        self.vec_z
    }
    //点乘
    pub fn dot(&self, other: &Vector) -> FloatType {
        self.vec_x * other.vec_x + self.vec_y * other.vec_y + self.vec_z * other.vec_z
    }

    //叉乘
    pub fn cross(&self, other: &Vector) -> Vector {
        Vector::new(
            self.vec_y * other.vec_z - self.vec_z * other.vec_y,
            self.vec_z * other.vec_x - self.vec_x * other.vec_z,
            self.vec_x * other.vec_y - self.vec_y * other.vec_x,
        )
    }

    //投影
    pub fn project(&self, onto: &Vector) -> Vector {
        let scalar = self.dot(onto) / onto.dot(onto);
        Vector::new(
            onto.vec_x * scalar,
            onto.vec_y * scalar,
            onto.vec_z * scalar,
        )
    }
    // 标量加法
    pub fn add_scalar(&self, scalar: FloatType) -> Vector {
        Vector::new(
            self.vec_x + scalar,
            self.vec_y + scalar,
            self.vec_z + scalar,
        )
    }

    // 标量减法
    pub fn subtract_scalar(&self, scalar: FloatType) -> Vector {
        Vector::new(
            self.vec_x - scalar,
            self.vec_y - scalar,
            self.vec_z - scalar,
        )
    }

    // 标量乘法
    pub fn multiply_scalar(&self, scalar: FloatType) -> Vector {
        Vector::new(
            self.vec_x * scalar,
            self.vec_y * scalar,
            self.vec_z * scalar,
        )
    }

    // 标量除法
    pub fn divide_scalar(&self, scalar: FloatType) -> Vector {
        Vector::new(
            self.vec_x / scalar,
            self.vec_y / scalar,
            self.vec_z / scalar,
        )
    }
    // 求模
    pub fn magnitude(&self) -> FloatType {
        (self.vec_x * self.vec_x + self.vec_y * self.vec_y + self.vec_z * self.vec_z).sqrt()
    }

    // 单位化
    pub fn normalize(&self) -> Vector {
        let magnitude = self.magnitude();
        Vector::new(
            self.vec_x / magnitude,
            self.vec_y / magnitude,
            self.vec_z / magnitude,
        )
    }

    // 计算角度
    pub fn angle(&self, other: &Vector) -> FloatType {
        let dot_product = self.dot(other);
        let magnitude_product = self.magnitude() * other.magnitude();
        dot_product.acos() / magnitude_product
    }
}