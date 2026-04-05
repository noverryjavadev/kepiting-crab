fn main() {
    println!("Hello, world!");
}

#[test]
fn test() {
    let x = 5;
    let y = 10;


    println!("x: {}, y: {}, z: {}", x, y, x + y);
}

#[test]
fn hello_test(){
    println!("Hello, world!");
}

#[test]
fn hello_world(){
    println!("Hello, world!");
    println!("Hello, world!");
    println!("=============");

    print!("HELLO");
    print!("HELLO");
}

#[test]
fn test_variables(){
    let x = 5;
    let y = 10;
    let name = "Bambang Soeharto";
    println!("x: {}, y: {}, z: {}", x, y, x + y);
    println!("name : {}", name);

}

#[test]
fn test_shadowing(){
    let name = "Bambang Soharto";
    println!("name: {}", name);

    let name = 1234;
    println!("name: {}", name);


}


#[test]
fn test_comment(){
    // contoh comment

    /*comment lagi*/

    /*coba lagi
    comment*/
}

#[test]
fn data_type(){
    let age: i32 = 18;
    println!("age: {}", age);
}

#[test]
fn number() {
    let age: i32 = 18;
    println!("age: {}", age);

    let b: f64 = 10.2;
    println!("b: {}", b);
}

#[test]
fn number_conversion() {
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = b as i32;
    println!("{}", c);

    let d: i64 = 1000000000;
    let e: i8 = d as i8;
    println!("{}", e);


    println!("----------------");

    let az: i8 = 20;
    println!("{}",az);

    let bz: i16 = az as i16;
    println!("{}", bz);


}

#[test]
fn numeric_operator() {
    let a = 10;
    let b = 10;
    let c = a * b;
    println!("{}", c);
    let d = a / b;
    println!("{}", d);
    let e = a + b;
    println!("{}", e);
}

#[test]
fn comparisson_operator(){
    let a = 10;
    let b = 20;
    let c = a > b;
    println!("{}", c);
}