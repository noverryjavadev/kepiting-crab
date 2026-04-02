use std::arch::aarch64::int32x2_t;

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
fn number(){
    let age: i32 = 18;
    println!("age: {}", age);


    let b: f64 = 10.2;
    println!("b: {}", b);
}