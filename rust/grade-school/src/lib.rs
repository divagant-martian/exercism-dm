use std::collections::HashMap;
pub struct School {
    info:HashMap<u32,Vec<String>> // good enough
}

impl School {
    pub fn new() -> School {
        School{ info:HashMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.info.entry(grade).or_insert(Vec::new()).push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut x = self.info.keys().map(|&x| x).collect::<Vec<u32>>();
        x.sort();
        x
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        if !self.info.contains_key(&grade) {return None;}
        let mut x = self.info.get(&grade).unwrap().clone();
        x.sort();
        Some(x)
    }
}
