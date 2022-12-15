fn main() {

    // IF AND ELSE

    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number_one = 3;

    if number_one != 0 {
        println!("Number was something other than zero");
    }

    // Handling Multiple Conditions with else if

    let number_two = 6;

    if number_two % 4 == 0 {
        println!("number is divisible by 4");
    } else if number_two % 3 == 0 {
        println!("number is divisible by 3");
    } else if number_two % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let statement 

    let condition = true;
    let number = if condition { 5 } else { 6 };
    //let number = if condition { 5 } else { "six" };  produce a error because we have a two different types

    println!("The value of number is: {number}");


    // REPETITION WITH LOOPS
    

    // infinite loop
    /*loop{
        println!("again!");
    }*/


    // Returning Values from Loops


    let mut counter = 0;

    let result = loop{
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    //Loop Labels to Disambiguate Between Multiple Loops


    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop{
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");


    // Conditional Loops with while

    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");


    // Looping Through a Collection with for

    // Example with while  (UNSAFE) 
    println!("WHILE safe");
    let a_one = [10, 20, 30, 40, 50];
    let mut index_for = 0;

    while index_for < 5 {
        println!("the value is: {}", a_one[index_for]);

        index_for += 1;
    }

    println!("FOR safe");
    // Example with For  (SAFE)

    let a_two = [10, 20, 30, 40, 50];

    for element in a_two {
        println!("the value is: {element}");
    }

}
