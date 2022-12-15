fn main() {

    // VARIABLES AND MUTABILITY

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("The value is : {THREE_HOURS_IN_SECONDS}");

    //Shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the scope is:{x}");
    }

    println!("The value of x is: {x}");

    // OK
    
    let spaces = "    " ;
    let spaces = spaces.len();

    // Error
    /*let mut spaces = "   ";
    spaces = spaces.len();*/


    // DATA TYPES
    
    // Tuples type
    let guess: u32 = "32".parse().expect("Not a number!");


    let tup: (i32,f64,u8) = (500,6.4,1);
    let first = tup.0;
    println!("tuples {first}");

    // Array Type
    let a = [1,2,3,4,5]; // Infer type

    let b: [i32; 5] = [1,2,3,4,5]; // Explicit declaration

    let a = [3;5]; // Contain 5 elements that will print value 3


    // Accesing array elements

    let first = a[0];

    let second = a[1];


    // FUNCTIONS

    

}
