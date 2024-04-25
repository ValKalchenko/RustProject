use std::collections::HashMap;
use std::collections::HashSet;
struct Student {
    name:String,
    school:String,
    age:u32
 }

 #[derive(Debug)]
 enum School {
    Student,Professor
 }

fn main() {
    println!("Hello, world!!");

    println!();

    // boolean logic
    //variable named door_locked set to true
    let door_locked = true;
    let lights_on = false;

    // if condition
    // if you want to use the NOT operator use !door_locked
    if !door_locked {
        println!("The door is locked! Use the key");
    } else {
        println!("The door is not locked! Open it");
    }

    if lights_on {
        println!("The lights are on!");
    } else {
        println!("The lights are not on (off)");
    }

    println!();

    // conbine into multiple conditions, joining 2 statements
    // parenthesis before if/else if are optional
    if !door_locked && lights_on {
        println!("The door is not locked and the lights are on!");
    } else if !door_locked && !lights_on {
        println!("The door is not locked and the lights are off!");
    } else if door_locked && lights_on {
        println!("The door is locked and the lights are on!")
    } else {
        println!("The door is locked and the lights are not on!")
    }

    println!();

    // or statements 
    let watch_tv = false;
    let read_book = true;

    if watch_tv || read_book {
        println!("You can watch tv or read a book");
    } else {
        println!("Time to go to bed")
    }

    println!();

    // less than or greater than or equal
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else if n == 0 {
        println!("{} is zero", n);
    }

    println!();

    // for loop
    // "in" is a type of iterator 
    // 1..15 is a range

    for num in 1..15 {
        if num % 15 == 0 {
            println!("fizzbuzz");
        } else if num % 3 == 0 {
            println!("fizz");
        } else if num % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", num);
        }
    }

    println!();

    // indefinite loop
    // mut is a keyword which means mutability (indicates if a variable can be changed (or mutated))
    let mut count = 0;

    loop {
        count +=1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("Ok, that's enough");
            break;
        }
    }

    println!();

    // while loop
    let mut number = 1;

    while number < 20 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if number % 3 == 0 {
            println!("fizz");
        } else if number % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", number)
        }

        // increment counter
        number += 1;
    }

    println!();

    // match statement
    // state_code is matched with a list of values 
    // if a match is found, a string value is returned to variable state
    let state_code = "PA";
    let state = match state_code {
        "PA" => {println!("Found match for PA"); "Pennsylvania"},
        "CA" => "California",
        _ => "Unknown"
    };
    println!("State name is {}",state);

    println!();

    // Array

    let arr = [10,20,30,40];
    println!("array is {:?}",arr); // {:?} used to print values inside of array
    println!("array size is :{}",arr.len());

    println!();
    
    // Slices
    // A slice is a pointer to a block of memory
    // Syntax: let sliced_value = &data_structure[start_index..end_index]

    let n1 = "Programming".to_string(); // converts string to object type
    println!("length of string is {}",n1.len());
    let c1 = &n1[4..9]; // slicer
   
    // fetches characters at 4,5,6,7, and 8 indexes
    println!("{}",c1);

    println!();
    // Vectors
    // resizable data structure, can shrink or grow
    // vector creation with vec! macro
    let v = vec![1, 2, 3];
    
    println!("v= {:?}", v);

    println!();
    // Hash map
    // store data in key-value pairs
    // each key is unique but values can duplicate
    // values can be accessed by their key
    let mut fruit: HashMap<i32, String> = HashMap::new();
    fruit.insert(1, String::from("Apple"));
    fruit.insert(2, String::from("Pear"));
    
    println!("HashMap = {:?}", fruit);

    println!();

    // Hash set
    // allows storage of values without duplicates
    let mut colors: HashSet<&str> = HashSet::new();
    
    // insert values in a HashSet
    colors.insert("Red");
    colors.insert("Yellow");
    colors.insert("Green");
    colors.insert("Red"); // duplicate value will not be printed 

    println!("colors = {:?}", colors);

    println!();
    
    // Tuples
    // compound data type
    // can store multiple values of different types
    // Syntax: let tuple_name:(data_type1,data_type2,data_type3) = (value1,value2,value3);

    let tuple:(i32,bool,f64) = (-325,false,2.5);
    println!("{:?}",tuple);

    println!();

    // Structs
    // allows users combine data items of different types

    let stu1 = Student {
        school:String::from("CHC"),
        name:String::from("Valerie"),
        age:22
     };
     println!("Name is {}, School is {}, Age is {}",stu1.name,stu1.school,stu1.age);


     println!();
    
    // Enums
    // used when selecting a value from a list of variants

    let student = School::Student;
    let professor = School::Professor;

    println!("{:?}",student);
    println!("{:?}",professor);

}
