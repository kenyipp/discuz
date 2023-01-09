fn main() {
    // They are string literals.
    // Used when the value of a string is known at compile time.
    let company: &str = "Hello world";
    let location: &str = "Canada";
    println!("Company is: {} & location is {}", company, location);
    // String object
    // Used when the value is provided at runtime
    let empty_string = String::new();
    println!("The length of the empty_string is {}", empty_string.len());
    let mut content_string = String::from("Tutorial point");
    println!("The length of the content_string is {}", content_string.len());
    content_string = String::from("Another content");
    println!("The length of the content_string changed to {} now!", content_string.len());
    // Condition flow
    let num = 12;
    if num % 2 == 0 {
        println!("The num is an even number");
    } else {
        println!("The num is an odd number");
    }
    // Match (like switch in javascript)
    let state_code = "MH";
    let state = match state_code {
        "MH" => {
            println!("Found match for MH");
            "Maharashtra"
        }
        "KL" => "Kerala",
        _ => "Hello",
    };
    println!("The full name of {} is {}", state_code, state);
    // Match with number
    let number = 13;
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
    }
    // Slices
    let string1 = "Tutorials";
    // The end_index will not be included in final string
    let string2 = &string1[2..4];
    println!("string1 is {} and string2 is {}", string1, string2);
    // Struct keyword is used to declare a structure
    // Since structures are statically typed, every field in the structure
    // must be associated with a data type
    struct Employee {
        company: String,
        name: String,
        age: u32,
    }
    let employee = Employee {
        company: String::from("TutorialPoint"),
        name: String::from("Ken Yip"),
        age: 50,
    };
    println!("The employee's name is {} and his age is {}", employee.name, employee.age);

    // The `derive` attribute automatically creates the implementation
    // required to make this `struct` printable with `fmt::Debug`
    #[derive(Debug)]
    enum Gender {
        Male,
        Female,
    }

    let male = Gender::Male;
    println!("{:?}", male);

    #[derive(Debug)]
    struct Person {
        name: String,
        gender: Gender,
    }

    let person1 = Person {
        name: String::from("Amy"),
        gender: Gender::Female,
    };

    println!("{:?}", person1);

}
