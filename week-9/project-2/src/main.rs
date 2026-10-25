use std::fs::File;
use std::io::Write;

fn main() {
let students = vec![
("Oluchi Mordi", "ACC10211111", "Accounting", "300"),
("Adams Aliyu", "ECO10110101", "Economics", "100"),
("Shania Bolade", "CSC10328828", "Computer", "200"),
("Adekunle Gold", "EEE11020202", "Electrical", "200"),
("Bianca Edemoh", "ME10102001", "Mechanical", "100"),
];

println!("PAU STUDENT MANAGEMENT SYSTEM");

println!("Student Name | Matric Number | Department | Level");

for student in &students {
println!(
"{} | {} | {} | {}",
student.0, student.1, student.2, student.3
);
}

let mut file = File::create("students.txt")
.expect("Could not create file");

writeln!(file, "PAU STUDENT MANAGEMENT SYSTEM")
.expect("Could not write to file");

writeln!(
file,
"Student Name | Matric Number | Department | Level"
)
.expect("Could not write to file");

for student in &students {
writeln!(
file,
"{} | {} | {} | {}",
student.0, student.1, student.2, student.3
)
.expect("Could not write to file");
}

println!("Student details saved to students.txt");
}
