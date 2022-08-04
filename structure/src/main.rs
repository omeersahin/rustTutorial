struct Student{
    name:String,
    age:i32,
    grade:f32,
}



fn main() {
    let student1 = Student{
        name:String::from("Omer"),
        age:15,
        grade:88.7
    };
    println!("Student Name : {} \nAge : {} \nGrade : {}",student1.name,student1.age,student1.grade);
    let mut student2 = Student{
        name:String::from("Unknown"),
        age:10,
        grade:20.0
    };

    student2.age = 18;
    //println!("Student2 Name : {} \nAge : {} \nGrade : {}",student2.name,student2.age,student2.grade);

    print_student_info(student2);

    let mut student2 = Student{
        name:String::from("Unknown"),
        age:10,
        grade:20.0
    };
    println!("{} has better grade",better_grade(student1,student2).name);
}

fn print_student_info(student_t : Student){
    println!("Student Name : {} \nAge : {} \nGrade : {}",student_t.name,student_t.age,student_t.grade);
}

fn better_grade(student_t1 : Student,student_t2 : Student)->Student{
    if student_t1.grade>student_t2.grade {
        return student_t1;
    } else {
        return student_t2;
    }
}