fn main() {

    //Creating a New Vector
    let v: Vec<i32> = Vec::new();
    let b = vec![1,2,3];

    println!("{:?}", v);
    println!("{:?}", b);

    //Updating a Vector
    let mut n = Vec::new();

    n.push(5);
    n.push(6);
    n.push(7);
    n.push(8);
    n.push(9);

    println!("{:?}",n);


    // Reading Elements of Vectors
    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third{
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element.")
    }

    let _does_not_exist = &v[100];
    let _does_not_exist = v.get(100);

    let mut v = vec![1,2,3,4,5];

    let first = &v[0];

    v.push(6);

    //println!("The first element is : {first}");


    // 8.2 Storing UTF-8 Encoded Text with Strings

    // Creating a New String

    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    //Appending to a String with push_str and push

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    //Concatenation with the + Operator or the format! Macro

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    //fn add(self, s: &str) -> String {


        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
    
        let s = s1 + "-" + &s2 + "-" + &s3;

        //Internal Representation
        let hello = "Здравствуйте";
        let answer = &hello[0..1];


        // 8.3 Storing Keys with Associated Values in Hash Maps

        //Creating a New Hash Map

        use std::collections::HashMap;

        let mut scores = HashMap::new();
    
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);


        let team_name = String::from("Blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);


        for (key, value) in &scores {
            println!("{key}: {value}");
        }

        // Hash Maps and Ownership
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");
    
        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // field_name and field_value are invalid at this point, try using them and
        // see what compiler error you get!
}