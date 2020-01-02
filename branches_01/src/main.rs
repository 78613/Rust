

fn main() {
    test_01();
    test_02();
    //test_03();
    test_04();
    test_05();
    test_06();
    //test_07();

}

fn test_01() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn test_02() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

/*
fn test_03() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
*/

fn test_04() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}

fn test_05() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn test_06() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}

/*
fn test_07() {
    let condition = true;

    let number = if condition {
        5
    } else {
        "six"
    };

    println!("The value of number is: {}", number);
}
*/