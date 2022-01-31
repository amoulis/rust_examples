//use std::fmt;
//use std::fmt::Display;

fn add_value(foo: u32, bar: u8) -> u32 { // -> is for return value
    let mut foobar = 0; // mut => mutable variable, value can be re-assigned
    foobar = foo + u32::from(bar); // conversion from u8 to u32
    return foobar;
}

fn add_value_8bits(foo: u8, bar: u8) -> u8 { // -> is for return value
    let mut foobar = 0; // mut => mutable variable, value can be re-assigned
    foobar = foo + bar; 
    return foobar;
}

//fn add_value_32bits(foo: u32, bar: u32) -> u32 { // -> is for return value
//    let mut foobar = 0; // mut => mutable variable, value can be re-assigned
//    foobar = foo + bar; 
//    return foobar;
//}

fn main() {
   let magikarp: u8 = 5; // No type declaration and not a mut variable
   let pikachu = 255;
   let pikachu8: u8 = 255;
   let mut ret;
   let mut ret8;
   
   ret = add_value(pikachu, magikarp);
   println!("And ret is : {}", ret);
   
   //overflow : thread 'main' panicked at 'attempt to add with overflow', src/main.rs:12:14
   ret8 = add_value_8bits(pikachu8, magikarp);
   println!("And ret overflow is : {}", ret8);
   
   //ret = add_value_32bits(pikachu, magikarp);
   //println!("And ret overflow is : {}", ret);
   //
}
