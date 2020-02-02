pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        if row_count == 0 {
            return PascalsTriangle { rows: vec![] };
        }

        let row_count = row_count as usize;
        let mut rows = Vec::with_capacity(row_count);
        rows.push(vec![1]);

        let mut last_row = &rows[0];

        for i in 1..row_count {
            let mut current = Vec::with_capacity(i + 1);
            current.push(1);
            for j in 1..i {
                current.push(last_row[j - 1] + last_row[j]);
            }
            current.push(1);
            rows.push(current);
            last_row = &rows[i];
        }

        PascalsTriangle { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
