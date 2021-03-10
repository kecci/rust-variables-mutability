fn main() {
    /* Immutable */ 
    let age = 27;
    println!("My age is {}", age);
    // age = 26;
    // println!("No no my actual age is {}", age);

    /* Mutable */ 
    let mut age = 27;
    println!("My age is {}", age);
    age = 26;
    println!("No no my actual age is {}", age);

    /* Constant */ 
    const IS_VIP: bool = true;
    println!("Is VIP user: {}", IS_VIP);

    /* Data Type */ 
    // Variable will know the data type
    let name = "Ali"; // ----> type: string
    println!("{}", name);
    
    let age = 27; // ---> type: integer
    println!("{}", age);
    
    let pi = 3.14; // ---> type: float
    println!("{}", pi);

    // Constant should declare the data type
    // const IS_VIP = true;

    /* Shadowing */
    let age = 26;
    let age = age + 1;
    println!("My age is: {}", age);

    /* Shadowing and Mutability */
    let age = 27;
    println!("{}", age);
    let age = "Ali";
    println!("{}", age);

    let mut name = "Ali";
    println!("{}", name);
    // You can't do the same thing without shadowing variable
    // name = name.len();
    // println!("My name is {}", name);
    name = "John";

    // You can do the same thing with shadowing variable
    let name = name.len();
    println!("{}", name);
}
