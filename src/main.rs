fn main() {
    let audience = "World";
    greeting_function(audience);
    expression_example ();
}

fn greeting_function(name: &str) {
    println!("Hello, {}", name);
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn expression_example (){
    let y = {
        let x = 3;
        x + 1 // no ending semi colon, otherwise it becomes a statement;
    };

    println!("The value of y is: {}", y);

    let a = 0.675;
    let b = -0.324;
    let sum = add_floats(a , b);
    println!("{} + {} = {}", a, b, sum);
}

fn add_floats(a:f64 , b:f64) -> f64{
    a + b
}