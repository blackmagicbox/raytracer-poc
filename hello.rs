fn main() {
    let x = 10;
    println!("X is {}", x);
    println!("In the example above the {0} was how use template strings like x = {1}!", "topic", x);
    println!("It's also possible to format while printing for instance {0} in binary is {1:b}", x, x);

    // Let's try capture the surrounding variable
    let number:f64 = 1.0;
    let width: usize = 5;
    println!("The value is {number:>width$}");
}