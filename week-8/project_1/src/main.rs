fn main() {
    let levels = vec![
        "APS 1-2",
        "APS 3-5",
        "APS 5-8",
        "EL1 8-10",
        "EL2 10-13",
        "SES",
    ];

    let office_admin = vec![
        "Intern",
        "Administrator",
        "Senior Administrator",
        "Office Manager",
        "Director",
        "CEO",
    ];

    let academic = vec![
        "-",
        "Research Assistant",
        "PhD Candidate",
        "Post-Doc Researcher",
        "Senior Lecturer",
        "Dean",
    ];

    let lawyer = vec![
        "Paralegal",
        "Junior Associate",
        "Associate",
        "Senior Associate 1-2",
        "Senior Associate 3-4",
        "Partner",
    ];

    let teacher = vec![
        "Placement",
        "Classroom Teacher",
        "Senior Teacher",
        "Leading Teacher",
        "Deputy Principal",
        "Principal",
    ];

    let profession = "Lawyer";
    let role = "Associate";

    let mut i = 0;
    let mut found = false;

    if profession == "Office Admin" {
        while i < office_admin.len() {
            if office_admin[i] == role {
                println!("Level: {}", levels[i]);
                found = true;
                break;
            }
            i = i + 1;
        }
    } else if profession == "Academic" {
        while i < academic.len() {
            if academic[i] == role {
                println!("Level: {}", levels[i]);
                found = true;
                break;
            }
            i = i + 1;
        }
    } else if profession == "Lawyer" {
        while i < lawyer.len() {
            if lawyer[i] == role {
                println!("Level: {}", levels[i]);
                found = true;
                break;
            }
            i = i + 1;
        }
    } else if profession == "Teacher" {
        while i < teacher.len() {
            if teacher[i] == role {
                println!("Level: {}", levels[i]);
                found = true;
                break;
            }
            i = i + 1;
        }
    }

    if found == false {
        println!("Invalid role or profession");
    }
}