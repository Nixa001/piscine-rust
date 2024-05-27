#[derive(Debug, PartialEq,Eq)]

pub struct Student (    
    pub u32,
    pub String,
    pub String,
);
pub fn id(student: &Student) -> u32 {
    student.0
}

pub fn first_name(student: &Student) -> String {
    student.1.to_string()
}

pub fn last_name(student: &Student) -> String {
    student.2.to_string()
}

// fn main() {
// 	let student = Student(20, "Pedro".to_string(), "Domingos".to_string());
// 	println!("Student: {:?}", student);
// 	println!("Student first name: {}", first_name(&student));
// 	println!("Student last name: {}", last_name(&student));
// 	println!("Student Id: {}", id(&student));
// }
