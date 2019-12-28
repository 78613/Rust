//#[macro_use]
//extern crate memoffset;


fn main() {
    shadowing();
    scalars();
    floating_point();
    chars();
    tuples();
    arrays();
}

fn shadowing() {
    // This is a very interesting property.
    // See shadowing documentation for details.  
    {
        let x = 5;
        let x = x + 1;
        let x = x * 2;    
        println!("The value of x is: {}", x);
    
        //x = 6;
        println!("The value of x is: {}", x);
    }
}
 
fn scalars() {
    // Scalar Types
    {
        println!{"\n"};
        let tmp: u32 = (4 * 256) - 1;
    
        println!("tmp sizeof {}", std::mem::size_of::<i32>());
        println!("tmp sizeof {}", std::mem::size_of::<i64>());
        println!("tmp sizeof {}", std::mem::size_of_val(&tmp));
    
        println!("tmp in {} = {}", "Decimal", tmp);
        println!("tmp in {} = {}", "Decimal", tmp);
    
        println!("tmp in {} = {}", "Hex", tmp);
        println!("tmp in {} = {:X}", "Hex", tmp);
        println!("tmp in {} = 0x{:X}", "Hex", tmp);
        println!("tmp in {} = 0x{:X?}", "Hex", tmp);
        println!("tmp in {} = 0x{:X?}", "Hex", tmp);
        println!("tmp in {} = {:x}", "Hex", tmp);
        println!("tmp in {} = 0x{:x}", "Hex", tmp);
        println!("tmp in {} = 0x{:32x?}", "Hex", tmp);
    
        println!("tmp in {} = {:0b}",   "bin", tmp);
        println!("tmp in {} = {:04b}", "bin", tmp);
        println!("tmp in {} = {:08b}", "bin", tmp);
        println!("tmp in {} = {:016b}", "bin", tmp);
        println!("tmp in {} = {:032b}", "bin", tmp);
        println!("tmp in {} = {:064b}", "bin", tmp);
    
        //println!("{}:{} {}()", line!(), file!(), func!());
        println!("{}", line!());
        println!("{}", column!());
        println!("{}", file!());
        println!("{}", module_path!());
        // No supported way to get the function name for print debugging...
        //println!("{}", concat!(module_path!(), "::", function!()));
    
        println!("L-{:05}: M-{:20} F-{:20} ", 
            line!(), module_path!(), file!());
    }
}

fn floating_point() {
    // Floating point
    {
        let a = 2.0;
        let b: f64 = 3.0;
        println!("{} = {}", "a", a);
        println!("{} = {}", "a", a/3.3);
        println!("{} = {}", "b", b);
        println!("{} = {}", "b", b/2.2);
    }
}

fn chars() {
    // char's are 32b wide!!!
    {
        let c: char = ' ';
        let u: u8   = 0;
        println!("tmp sizeof {}", std::mem::size_of_val(&c));
        println!("tmp sizeof {}", std::mem::size_of_val(&u));
    }
}

fn tuples() {
    // Tuples
    {
        let x: (i32, f64, u8) = (500, 6.4, 1);
        let _five_hundred = x.0;
        let _six_point_four = x.1;
        let _one = x.2;

        println!("tup sizeof {}", std::mem::size_of_val(&x));
        println!("tup sizeof {}", std::mem::size_of_val(&x.0));
        println!("tup sizeof {}", std::mem::size_of_val(&x.1));
        println!("tup sizeof {}", std::mem::size_of_val(&x.2));

        println!("{}", x.0);
        println!("{}", x.1);
        println!("{}", x.2);
    }

    // Tuples
    {
        let tup = (500, 6.4, 1);
        let (x, y, z) = tup;

        println!("tup sizeof {}", std::mem::size_of_val(&tup));

        println!("{}", x);
        println!("{}", y);
        println!("{}", z);
    }

    //offsetof
    {
        //#[macro_use]
        //extern crate memoffset;

        //let foo = (500, 6.4, 1);
        //println!("offset: {} = {}", "foo.0", offset_of!(foo, 0));
    }
}

fn arrays() {
    //Array
    {
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        println!("a sizeof {}", std::mem::size_of_val(&a));
        println!("a[0] sizeof {}", std::mem::size_of_val(&a[0]));
        println!("a[0] {}", a[0]);

        let b = [3; 5];
        println!("b sizeof {}", std::mem::size_of_val(&b));
        println!("b[0] sizeof {}", std::mem::size_of_val(&b[0]));
        println!("b[0] {}", b[0]);
    }

    //invalid access
    {
        let a = [1, 2, 3, 4, 5];
        let index = 10;
   
        let element = 0;
        //let element = a[index];  // runtime time error
        //let element = a[10];   //compile time error
    
        println!("The value of element is: {}", element);
    }

}
