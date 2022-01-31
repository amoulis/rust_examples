use std::fmt;
use std::fmt::Display;

// Struct definition
struct Hero(u32, u32);
// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Hero {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} {}", self.0, self.1)
    }
}

//Struct with namables fields
struct Avengers {
   IronMan : u8,
   Hulk : u8,
   Thor : u16,
    
}
impl fmt::Display for Avengers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.IronMan, self.Hulk, self.Thor)
    }
}


fn main() {
   let foo: u16 = 36;
   
   let  batman = Hero(1u32, 2u32);
   let MightiestHeroes = Avengers { IronMan: 1, Hulk: 3, Thor: 255 };
   
   println!("Hello, world {}!", batman);
   println!("Hello, world {}!", MightiestHeroes);


   // Tuple Manipulation
   let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // Values can be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);
    println!("tuple of tuples: {:?}", long_tuple);
}
