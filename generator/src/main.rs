/*
fn main() {
    println!("Hello, world!");
}
*/

//extern crate rand;

//use rand::Rng;

/*
fn main() {
    let mut rng = rand::thread_rng();

    /*
    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();

    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
    */

    println!("value: {}", rng.gen_range(0, 100_000));

}
*/


use std::io::Write;
use std::fs::File;
use rand::Rng;

fn main() -> std::io::Result<()> {
    let mut buffer = File::create("foo.txt")?;
    let mut rng = rand::thread_rng();
    
    let val = rng.gen_range(0, 100_000);

    println!("rand: {}", val);

    //buffer.write_all(b"some bytes")?;
    //buffer.flush()?;

    write!(&mut buffer, "{}", val)?;
    
    Ok(())
}
