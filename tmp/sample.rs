fn main() {
    // Integer addition
    println!("1 + 2 = {}", 1 + 2);
    println!("1 + 24 = {}", 1 + 24);
    println!("1 + 24 = 0x{:X?}", 1 + 24);
    println!("1 + 24 = {:X}", 1 + 24);
    //println!("1 + 24 = {:p}", 1 + 24);
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1 - 2);
    println!("1 - 2 = {}", 1i32 - 2);
    //println!("1 - 2 = {}", 1u32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);
    println!("NOT false is {}", !false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("0011 XOR 0101 is {:08b}", 0b0011u32 ^ 0b0101);
    println!("0011 XOR 0101 is {:016b}", 0b0011u32 ^ 0b0101);
    println!("0011 XOR 0101 is {:032b}", 0b0011u32 ^ 0b0101);
    println!("0011 XOR 0101 is {:064b}b", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}
