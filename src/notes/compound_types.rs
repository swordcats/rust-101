#![allow(unused)] // allow for unused variables; no errors

fn main() {
    // Compound Types - Tuples & Arrays

    // TUPLES
    let student_a = ("Heath", 'A', 3.58);
    // Store up to 12? I think?
    //let name_student_a = student_a.0; // zero-based indexes
    //let grade_student_a = student_a.1;
    //let gpa_student_a = student_a.2;

    let (name_student_a, grade_student_a, gpa_student_a) = student_a;

    println!("My name is {}, my grade is {}, and my overall GPA is {}", name_student_a, grade_student_a, gpa_student_a);


    // ARRAYS - Use brackets []
    // Can store up to 32 values -- these values have to be similar data types
    // Have a fixed size that cannot be changed at runtime
    // Use a Vec (vector) if you want to change the sizee
    // Often used to represent fized-size data structures, like matrices or board games
    let students = ["Heath", "Bob", "Linda"];
    println!("The first student in our array is {}", students[0]); // tuples use dot notoation, arrays use bracket notation

    // SLICES
    let mut arr = [1,2,3,4,5];
    let slice = &mut arr[1..3]; // grabseleemnt at position 1 and element at position 2

    slice[0] = 6;
    slice[1] = 7;

    // Can add the mut keyword to make the array mutable; if slices are changed, array is changed

    println!("{:?}", arr); 

}