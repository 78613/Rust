fn main() {
    println!("Hello, world!");
    another_function(5, 6);

    let x = 99;
    another_function(x, 6);

    func_assign();
    func_expression();
    func_five();
    func_plus_one();
}    

fn another_function(x: i32, y: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x);    
    println!("The value of y is: {}", y);    
}

fn func_assign() {
    let mut x = 0;
    let mut y = 0;

    //comment out to see modify before read compiler warning.
    println!("The value of x is: {}", x);    
    println!("The value of y is: {}", y);    

    x = 1;
    y = 1;
    println!("The value of x is: {}", x);    
    println!("The value of y is: {}", y);    
}

fn func_expression()
{
    let x = 5;
    let y = {
        //let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);    
}

fn five() -> i32 {
    5
}

fn func_five() {
    let x = five();
    println!("The value of x is: {}", x);    
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn func_plus_one() {
    let x = plus_one(5);
    println!("The value of x is: {}", x);    
}
