use std::collections::HashMap;

pub struct School {
    grades: HashMap<u32, Vec<String>>, 
}

impl School {
    pub fn new() -> School {
        Self { grades: HashMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let students = self.grades.entry(grade).or_insert(vec![]);
        students.push(student.to_string());
        students.sort();
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut vec: Vec<u32> = self.grades.keys().map(|&k| k).collect();
        vec.sort();
        vec
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.grades.get(&grade).map(|grade| grade.clone())
    }
}
