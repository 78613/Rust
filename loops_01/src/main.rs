fn main() {
    //test_01();
    test_02();
    test_03();
    test_04();
    test_05();
    test_06();
}


/*
fn test_01() {
    loop {
        println!("again!");
    }
}
*/

fn test_02() {
    let mut counter = 0;

    // result is assigned the expression return of loop
    let result = loop {
        counter += 1;

        if counter == 10 {
            // break can contain an assigned value
            // the calculation below is the output of loop.  Assigned to result for test_02  visibility.
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn test_03() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

/*
  But this approach is error prone; we could cause the program to panic if the index length is 
  incorrect. It’s also slow, because the compiler adds runtime code to perform the conditional 
  check on every element on every iteration through the loop.
*/
fn test_04() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

/*
  we’ve now increased the safety of the code and eliminated the chance of bugs that might result 
  from going beyond the end of the array or not going far enough and missing some items.

  FYI: in test_04 example, one should never decouple iteration count from number of elements in 
  the array.  That's bacd practice no compiler can protect from.  The correct approach is:
    elems = sizeof(a) / sizeof(a[0]); //safety
    while (index < elems) {
        do_something();
        elems++;
    }

  The one very nice property of for() in RUST is that it makes it simpler for the compiler to infer
  bounds/limits and generates more performant code without the need for sanity code in each iteration.
*/
fn test_05() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}


fn test_06() {
    // note how "rev" reversed the ramge :-).
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}