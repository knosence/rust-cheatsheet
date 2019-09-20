use std::mem; //importing causes warnings if it is not used. gg line 90

fn main() {
    //Print Macro! to print and add a new line.
    println!("Hello, world!");
    
    let _x = 5; //unassigned integer.
    //x = 6; -> will not work because veriables are immunitable by default

    //to make it mutable use the `mut` key word.
    let mut _y = 5;
    _y = 6; //now works

    //we can assign veriable types by...
    let _z: u32 = 5;

    /*
     * the premative types are `i8, u8, i16, u16, i32, u32, i64, u64, i128, u128`
     * `isize, usize` are based on the computer.
     *    if 64, then default to 64
     *    if 32, then default to 32
     * floats are `f32, f64`
     *    f32 are single precision
     *   f64 are double precision
     */
    
    //Arithmetic
    let _a = 1 + 20;     //Addition
    let _s = 30 - 20;    //Subtraction
    let _m = 5 * 10;     //Multiplication
    let _d = 4 / 6;      //Division
    let _r = 49 % 2;     //Remander or Modulous

    //booleans
    let _t: bool = true;
    let _f: bool = false;

    //chars
    let _c: char = 'c'; //chars are asci by default

    //tuples
    let t:(i32, f64, char) = (42, 6.12, 'j');
    //Destrusturing tuples
    let (_i, _f, _ch) = t;
    //underscores are used to skip or ignore
    let  (_,_,_ch) = t;
    //t.0, t.1, t.3 access items in tuple

    //arrays are single type
    let _a = [1,2,3,4,5,6,7,8]; 
    let _a1 = _a[0];
    /*
     * arrays are allocated on the stack
     * arrays have a fixed size 
     * and can not be changed even when using `mut` key word.
     */

    //tuples inside of tuples
    let _t = (1, 'a', false);
    let _f = (2, _t, true);
    //Display tuples
    println!("{} {} {}", _t.0, _t.1, _t.2);

    // to Print a tuple inside of a tuple, use the `{:?}` debug flag
    println!("{:?}", _f); // Displays: (2(1,'a', false), true);

    // to pretty print use `{:#?}`
    println!("{:#?}", _f);
    /* Displays:
     * (
     *   2,
     *   (
     *       1,
     *       'a',
     *       false,
     *   ), 
     *   true,
     * )
     */ 

    let _new_t = (1,2,3,4,5,6,7,8,9,10,11,12,13);
    //`println!('{:?}', new_t)` will not work because the tuple is too long

    //Arrays
    let _xs: [i32; 5] = [4,5,6,7,8];
    println!("{} {}", _xs[0], _xs.len()); //`xs.len()` displays the size of array
    //Displays: (4, 5)
    println!("{}", mem::size_of_val(&_xs)); //gives the total size in memory.
    //Displays: (20)

    //slice
    let _ys = &_xs[2..4]; //4 is non-inclusive
    //indexes 2 and 3 are stored in _ys
    
    println!("{} {}", _ys[0], _ys[1]);
    //prints specific indexs.
    
    println!("{:?} {:?}", _xs, _ys);
    //displays; [6,7] [4,5,6,7,8]

    //slice of a string
    let _str = "String"; //&str type
    let _sstr = String::from("String!"); //std::String::String type
    /*
     * Strings in rust are more like tuples and arrays
     * and compound types made of different chars.
     * 
     * `let s: "String".to_string();` converts to a string type.
    */

    //slice of a string
    let slice = &_sstr[0..4];
    println!("{}", slice); //Displays: Stri
    
    //Concatenating strings
    let h = String::from("Hello, ");
    let w = String::from("World!"); 
    let s = h + &w; // self, &String
    println!("{}", s);
    //Displays: Hello, World!

    //Empty tuple returns empty tuples
    let _empty_t = (); //unit value.

}   

