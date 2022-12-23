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

    println!("The first element is : {first}");



}