use std::io;

fn convert_to_integer(data: & String) ->i32 {
    let data: i32 = data.trim().parse::<i32>().unwrap();
    data
}

fn main() {
    /* 
    let mut name = "Gain"; // mut is mutable variable
    name = "Olga";
    println!("name: {}", name);
    */
    /* 
    let mut numberOne = String::new();
    io::stdin().read_line(&mut numberOne).expect("Error reading input numberOne");

    let mut numberTwo = String::new();
    io::stdin().read_line(&mut numberTwo).expect("Error reading input numberTwo");

    if convert_to_integer(&numberOne) > convert_to_integer(&numberTwo) {
        print!("numberOne {} is must of numberTwo {}", numberOne, numberTwo);
    } else {
        print!("numberTwo {} is must of numberOne {}", numberTwo, numberOne);
    } 
    */
    /* 
    let mut sum = 0;
    let mut value_input = String::new();
    io::stdin().read_line(&mut value_input).expect("Error reading input value_input");
    let mut value_i32 =  convert_to_integer(&value_input);
    while value_i32!= 0 {
        let mut r = value_i32 %10;
        sum = sum + r;
        value_i32 = value_i32 / 10;
    }

    print!("the sum the digits is {}", sum)
    */

    let mut averages = String::new();
    io::stdin().read_line(&mut averages).expect("Error reading input averages");
    let mut sum_rec = 0;
    let mut i = 0;
    while convert_to_integer(&averages) > i {
        let mut avg_student = String::new();
        io::stdin().read_line(&mut avg_student).expect("Error reading input averages");
        i += 1;
        if convert_to_integer(&avg_student) >= 3 && convert_to_integer(&avg_student) < 6 {
            sum_rec += 1;
        }

        print!("the number of students in recovery is {}", sum_rec)

    }



}


// types datas

//let x: I32 =  9; Integer
//let y: u32 = 10; not negative 
// let z: f32 = 11.4; floating point
// let w: bool = false;