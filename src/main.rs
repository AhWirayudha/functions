fn main() {
    println!("Hello, world!");

    another_function(5);
    println!("{}", default_param(5, 6));
    print_label_measurement('h', 42);
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

