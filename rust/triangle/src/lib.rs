pub struct Triangle{
    // sides:[i32; 3],
    eqs:usize,
}
impl Triangle{
    pub fn build(v:[u32; 3]) -> Result<Triangle, &'static str>{
        if v[0]+v[1]+v[2]==0 || v[0]+v[1]<v[2] || v[0]+v[2]<v[1] || v[1]+v[2]<v[0]{
            return Err("impossibru!!");
        }
        Ok(
            Triangle{
                eqs:if v[0] == v[1] {1} else {0}+
                    if v[0] == v[2] {1} else {0}+
                    if v[1] == v[2] {1} else {0},
            }
        )
    }
    pub fn is_isosceles(&self) -> bool{ self.eqs == 2 || self.eqs == 1 }
    pub fn is_scalene(&self) -> bool{ self.eqs == 0 }
    pub fn is_equilateral(&self) -> bool{ self.eqs == 3 }
}
