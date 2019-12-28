# Rust
Rust

surprises:
- No debug equivalent of __FUNCTION__ or __FUNC__
- sizeof() equivalent requires borrowing.
  let foo: u32 = 123;
  println!("size = {}", std::mem::size_of_val(&foo));

  
