pub struct PascalsTriangle{
    rows: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut r : Vec<Vec<u32>> = Vec::with_capacity(row_count as usize);
        let mut last = vec![1];
        if row_count == 0 {return PascalsTriangle{rows:vec![]}}
        r.push(last.clone());
        for _ in 1..row_count{
            let mut new = vec![1];
            for j in 0..last.len()-1{
                new.push(last[j]+last[j+1])
            }
            new.push(1);
            last = new;
            r.push(last.clone());
        }
        PascalsTriangle{rows:r}
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
