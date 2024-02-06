fn main() {
    println!("Hello, world!");

    // variables in rust are bydefault immutable
    let  x = 5;

    //to create immutable variables we use mut keyboard after let 
    println!("value of x is {x}");

    // if we reassign the value then
    //x=2;
    println!("value of x is {x}");

    // constant variables 
    const VALUE_OF_P: u32 = 3;
    println!("Value of p is {VALUE_OF_P}");

    let x =10;
    
    // It creates a new variable x by repeating let x = , taking the original value and adding 5 so
    // the value of x is 15
    // shadowing process 
    let x = x+5;
    {
        let x = x+5;
        println!("Inner scope value of x is : {x}");
    }
    println!("outer scope value of x is {x}");

}
