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

#[test]
fn augmented_assignment() {
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}

#[test]
fn boolean() {
    let a = true;
    let b: bool = false;

    println!("{} {}", a, b);
}

#[test]
fn boolean_operator() {
    let absen = 75;
    let nilai_akhir = 80;

    let lulus_absen: bool = absen >= 75;
    let lulus_nilai_akhir: bool = nilai_akhir >= 75;

    let lulus: bool = lulus_absen && lulus_nilai_akhir;
    println!("{}", lulus);
}

#[test]
fn char_type(){
    let char1 : char = 'a';
    let char2 : char = 'b';
    println!("char1: {}, char2: {}", char1, char2);
}

#[test]
fn tuple() {
    let mut data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    // let a = data.0;
    // let b = data.1;
    // let c = data.2;

    let (a, b, c) = data;
    println!("{} {} {}", a, b, c);

    data.0 = 20;
    data.1 = 20.5;
    data.2 = false;
    println!("{:?}", data);
}

fn unit() {
    println!("Hello");
}

#[test]
fn test_unit() {
    let result: () = unit();
    println!("{:?}", result);

    let test: () = ();
    println!("{:?}", test);
}

#[test]
fn array() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);

    array[0] = 10;
    array[1] = 20;
    println!("{:?}", array);

    let length = array.len();
    println!("{}", length);
}

#[test]
fn two_dimensional_array() {
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6]
    ];

    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{:?}", matrix[0][0]);
    println!("{:?}", matrix[0][1]);
    println!("{:?}", matrix[0][2]);
    println!("{:?}", matrix[1]);
    println!("{:?}", matrix[1][0]);
    println!("{:?}", matrix[1][1]);
    println!("{:?}", matrix[1][2]);
}

const MAXIMUM: i32 = 100;

#[test]
fn constant() {
    const MINIMUM: i32 = 0;
    println!("{} {}", MINIMUM, MAXIMUM);
}


#[test]
fn variable_scope(){
    println!("{}", MAXIMUM);

    let b = 1;

    {
        println!("{}", b);
        let s = 2;
        println!("{}", s);
    }

    /* Tidak bisa diakses (error) */
    // println!("{}", s);
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("Kurniawan");
    println!("{}, {}", a, b);
}

fn function_b() {
    let a = 10;
    let b = String::from("Eko");
    println!("{}, {}", a, b);
}

#[test]
fn string() {

    let name: &str = "   Bambang Soeharto   ";
    let trim: &str = name.trim();

    println!("{}", name);
    println!("{}", trim);

}

#[test]
fn string_type() {

    let mut name: String = String::from("Bambang Soeharto");
    println!("{}", name);

    name.push_str(" Prabowo");
    println!("{}", name);

    let Purnomo = name.replace(" Prabowo", " Purnomo");
    println!("{}", name);
    println!("{}", Purnomo);

}

#[test]
fn ownership_rules() {
    let a = 10;

    {
        let b = 10;
        println!("{}", b);
    }

    println!("{}", a);
}

#[test]
fn data_copy() {
    let a = 10;
    let b = a;

    println!("{} {}", a, b);
}

#[test]
fn ownership_movement() {
    let name1: String = String::from("Eko");
    println!("{}", name1);

    let name2: String = name1; // ownerhsip pindah ke name2
    println!("{}", name2);
    // println!("{}", name1);
}

#[test]
fn clone() {
    let name1 = String::from("Eko");
    let name2 = name1.clone();

    println!("{} {}", name1, name2);
}

#[test]
fn if_expression() {
    let value = 7;
    let result: &str = if value >= 8 {
        "Good"
    } else if value >= 6 {
        "Not Bad"
    } else if value >= 3 {
        "Bad"
    } else {
        "Very Bad"
    };

    println!("{}", result);
}

#[test]
fn loop_expression() {
    let mut counter = 0;
    loop {
        counter += 1;

        if counter > 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }

        println!("Counter : {}", counter);
    }
}

#[test]
fn fish_buzz(){
    let mut fish = 0;

    loop {
        fish += 1;

        if fish > 20 {
            break;
        } else if fish % 3 == 0 && fish % 5 == 0  {
            println!("FizzBuzz");
        } else if fish % 3 == 0 {
            println!("Fizz");
        } else if fish % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", fish);
        }
    }
}

#[test]
fn loop_label() {
    let mut number = 1;
    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i);
            i += 1;
            if i > 10 {
                break;
            }
        }
        number += 1;
    }

    #[test]
    fn while_loop() {
        let mut counter = 0;
        while counter <= 10 {
            if counter % 2 == 0 {
                println!("Counter : {}", counter);
            }

            counter += 1;
        }
    }
}





