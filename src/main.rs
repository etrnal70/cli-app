extern crate box_drawing;

struct Animal{
    age : i32,
    size : i32,
    name : String,
}

struct Person {
    name : String,
    age : i32,
}

fn main() {
    println!("Hello, world!");
    test();


    let lion = Animal{
        age : 12,
        size : 50,
        name : String::from("Bro"),
    };

    match lion {
        Animal{age,size,name} => {
            println!("Age : {}, Size : {}, Name : {}",age, name, size);
        },
    }


fn test(){
    let z = 32;
    let x = 64;
    
    let res = sum(z, x);

    println!("Test = {}", res);
}

fn sum(a : i32, b : i32) -> i32{
    a + b
}

fn multiply(x : f32, y : f32) -> f32 {
    x * y
}
