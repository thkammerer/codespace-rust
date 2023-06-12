use core::num;
use std::{io::Stdin, vec};

// -------------------------------------------------------------------------------
//      Data Types in Rust
//          - Scalar
//              - Integers
//                  - signed
//                      - i8, i16, i32, i84
//                  - unsigned
//                      - u8, u16, u32, u64
//  	        - Floats
//                  - f32, f64
//              - bool
//              - char
//          - String
//          - Tupel
//          - Array
//          - Vector
//-----------------------------------------------

fn main() {
    // ***** control structures *****
    // try_conditionals();
    // try_match();
    // try_loop_while();
    try_for_loop();

    // ***** ownership *****
    // try_using_references();
    // try_ownership_and_references();
    // try_simple_ownership();

    // ***** data types *****
    //    try_strings();
    //    try_tuples();
    //    try_array();
    //    try_vector();
    //    try_functions();
    //    try_input_from_std_io()
}

fn try_for_loop() {
    let mut some_vec = vec![1, 3, 7, 45, 888];
    for i in 0..=4 {
        println!( "{}", some_vec[i] );
    }
    for value in some_vec.iter() {
        println!( "{}", value );
    }

    for value in some_vec.iter_mut() {
        *value += 5;
        println!( "{}", value );
    }

    println!( "{:?}", some_vec );
}

fn try_loop_while() {
    // infinite loop
    //    loop {
    //        println!( "in the loop" );
    //    }

    let my_number = 12;
    let mut guess = false;

    println!("Guess my number which is betwwen 1 and 20");
    while !guess {
        let mut input: String = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        let number: u32 = input.trim().parse().expect("invalid input");
        if number == my_number {
            println!("You got it! My number was {}", my_number );
            guess = true;
        } else {
            if number < my_number {
                println!("number to small, try again");
            }
            else {
                println!("number to big, try again");
            }
        }
    }
}

fn try_match() {
    let some_romanic_sign = 'x';
    let mut value = 0;
    match some_romanic_sign {
        'I' | 'i' => value = 1,
        'V' | 'v' => value = 5,
        'X' | 'x' => value = 10,
        'L' | 'l' => value = 50,
        'C' | 'c' => value = 100,
        'D' | 'd' => value = 500,
        'M' | 'm' => value = 1000,
        _ => value = -1,
    }
    if value > 0 {
        println!(
            "The value of the romanic number {} is {}",
            some_romanic_sign, value
        );
    } else {
        println!("{} is not a romanic number!", some_romanic_sign);
    }
}

fn try_conditionals() {
    let my_number = 23;

    if my_number > 30 {
        println!("{} is greater than 30 ", my_number);
    } else {
        println!("{} is smaler than or equal to 30 ", my_number);
    }
}

fn try_using_references() {
    // references rules
    //     1 one mutable reference in a scope
    //     2 many immutable references
    //     3 mutable and immutable can not coexist
    //     4 scope of a reference
    //     5 data should not change when immutable references ar in scope

    let mut heap_num = vec![4, 5, 6];
    {
        let ref1 = &mut heap_num;
        //    let ref2 = &mut heap_num;       // is not in accordance with rule 1: one mutable reference in a scope!
        //    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);
    }
    {
        let ref1 = &heap_num;
        let ref2 = &heap_num; // in accordance to rule 2: many immutable references in a scope!
        println!("ref1: {:?}, ref2: {:?}", ref1, ref2);
    }

    {
        let ref1 = &mut heap_num;
        //        let ref2 = &heap_num;       // is not in accordance with rule 3: mutable and immutable can not coexist
        //        println!("ref1: {:?}, ref2: {:?}", ref1, ref2);
    }

    {
        let ref1 = &mut heap_num;
        println!("ref1: {:?}", ref1);
        let ref2 = &heap_num; // is in accordance with rule 3: mutable and immutable can not coexist, because the scope of ref1 ends after printing.
        println!("ref2: {:?}", ref2);
    }

    {
        heap_num.push(666);
        let ref1 = &heap_num;
        let ref2 = &heap_num;
        println!("ref1: {:?}, ref2: {:?}", ref1, ref2);
        heap_num.push(777);
        // println!("ref1: {:?}, ref2: {:?}", ref1, ref2); this is not in accordance to rule 5: data should not change when immutable references ar in scope
    }
}

fn stack_function(mut num: i32) {
    num = 666;
    println!("in heap_function, num is {}", num);
}

fn heap_function(vector: &mut Vec<i32>) {
    vector[2] = 777;
    println!("in heap_function vector: {:?}", vector);
}

fn try_ownership_and_references() {
    let mut vec_1 = vec![10, 20, 666];
    println!("vec_1 : {:?}", vec_1);
    let mut ref_1 = &mut vec_1;

    println!("ref_1 : {:?}", ref_1);
    ref_1[2] = 777;
    println!("ref_1 after setting index 2 to 777: {:?}", ref_1);

    let mut my_num = 1;
    stack_function(my_num);
    println!("in main, my_num is {}", my_num);

    let mut heap_vec = vec![3, 11, 666];
    heap_function(&mut heap_vec);
    println!("in main heap_vec : {:?}", heap_vec);
}

fn try_simple_ownership() {
    // Ownership rules
    //      - each value in Rust has a varibale that's called its owner;
    //      - there can be only one owner at a time;
    //      - when he owner goes out of scope, that value will be dropped.
    //
    let x = 32.6;
    let y = x; // ownership doesn't move here, because String f64 is a primitive datatype, here a copy was done
    println!("x: {}, y: {}", x, y);

    let s1 = String::from("abc");
    //    let s2 = s1;                              // ownership moves here, because String is a non-primitive datatype
    //    println!("s1: {}, s2: {}", s1, s2);       // here the compiler don't want to translate this!

    let s3: &String = &s1; // now the ownership does not move, because it is only a reference!
    println!("s1: {}, s2: {}", s1, s3);

    let s4 = s1.clone(); // clone makes a copy!
    println!("s1: {}, s2: {}", s1, s3);
}

fn try_strings() {
    let some_string = "Fixed length string";
    let mut growable_sting = String::from("This string will grow");

    println!("initialize string {}", growable_sting);
    growable_sting.push('s');
    println!("after pushing 's' {}", growable_sting);
    growable_sting.remove(growable_sting.len() - 1);
    println!("after removing last 's' {}", growable_sting);
    growable_sting.push_str(" and grow!");
    println!("after pushing ' and grow!' {}", growable_sting);
}

fn try_tuples() {
    let my_information = ("Thomas", "Kammerer", "30.06.1967");
    println!(
        "{last_name}, {first_name}, date of birth: {birthdata}",
        last_name = my_information.1,
        first_name = my_information.0,
        birthdata = my_information.2
    );

    let (first_name, last_name, date_of_birth) = my_information;
    println!(
        "{}, {}, date_of_birth: {}",
        last_name, first_name, date_of_birth
    );

    let nested_tuple = (5, 5.12, (3, 1, 0), "Thomas");
    println!("nested tuble = {:?}", nested_tuple);
    println!("getting a nested value (should be 1) {}", nested_tuple.2 .1);
}

fn try_array() {
    let mut number_array: [i32; 5] = [3, 6, 11, 21, 555];
    println!("number array = {:?}", number_array);
    number_array[4] = 666;
    println!(
        "number array after setting last element to 666 = {:?}",
        number_array
    );

    let array_with_same_elements = [17; 5];
    println!("array initialized with 17 = {:?}", array_with_same_elements);
    println!("array has {} elements", array_with_same_elements.len());
    println!(
        "array occupying {} bytes of memory",
        std::mem::size_of_val(&array_with_same_elements)
    );
}

fn try_vector() {
    let mut number_vector: Vec<i32> = vec![10, 33, 17];
    println!(
        "print out the vector initializes with 10, 33, 17 {:?}",
        number_vector
    );
    number_vector.push(666);
    println!("print out the vector after pushing 666 {:?}", number_vector);
    number_vector[0] = 6;
    println!(
        "print out the vector after setting index 0 to 6 {:?}",
        number_vector
    );
    let subset_numbers = &&number_vector[0..=1];
    println!("print subset of vector {:?}", subset_numbers);
    number_vector.remove(1);
    println!(
        "print out the vector after removing index 1 {:?}",
        number_vector
    );

    let mut check_index = number_vector.get(1);
    println!("number_vector.get( 1 ) results in  {:?}", check_index);

    let mut check_index = number_vector.get(100);
    println!("number_vector.get( 100 ) results in  {:?}", check_index);

    println!(
        "Is the value of 666 in the vector? {}",
        number_vector.contains(&666)
    );
}

fn try_input_from_std_io() {
    let result: f64 = read_input_from_std_io();
    println!("input from std io is {}", result);
}

fn try_functions() {
    let a = 10;
    let b = 22;

    function_with_inputs("a", a);
    function_with_inputs("b", b);
    let result = function_with_inputs_and_output(a, b);

    function_with_inputs("result", result);

    let (add, sub, mul) = function_with_inputs_and_multiple_outputs(a, b);
    function_with_inputs("addition", add);
    function_with_inputs("subtraction", sub);
    function_with_inputs("multiplication", mul);
}

fn function_with_inputs(name: &str, value: i32) {
    println!("the value of {} is {}", name, value);
}

fn function_with_inputs_and_output(a: i32, b: i32) -> i32 {
    a + b
}

fn function_with_inputs_and_multiple_outputs(a: i32, b: i32) -> (i32, i32, i32) {
    (a + b, a - b, a * b)
}

fn read_input_from_std_io() -> f64 {
    println!("please insert a floating point: ");

    let mut my_input: String = String::new();
    std::io::stdin()
        .read_line(&mut my_input)
        .expect("failed to read input");

    let formated_input: f64 = my_input.trim().parse().expect("invalid input");
    println!("{:?}", formated_input);
    formated_input
}
