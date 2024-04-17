fn main() {
    println!("Hello, world!");

    another_function(5);
    println!("{}", default_param(5, 6));
    print_label_measurement('h', 42);

    //statement
    let y = {
        let x = 3;
        x + 1 //expression, not end with semicolon if you end with semicolon, it will be a statement and statement will not return value
    };

    println!("The value of y is: {y}");

    let five = five();
    println!("The value of five is: {five}");

    let plus_one = plus_one(7);
    println!("The value of plus_one is: {plus_one}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}

fn another_function(x: i32) {
    println!("Another function. {x}");
}

//default parameters on function
fn default_param(x: i32, y: i32) -> i32 {
   println!("{} + {} = {}", x, y, x + y);
   x + y
}
//using char is different from string
fn print_label_measurement(label: char, measurement: i32) {
    println!("{label}: {measurement}");
}

