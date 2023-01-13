fn main() {
    let variable = 5;
    {
        let variable = 6;
        println!("The value of variable is: {}", variable);
    }
    println!("The value of variable is: {}", variable);

    let x = 10;
    {
        let y =  20;
        println!("The values of x and y is: {}, {}", x, y);
    }
    // println!("The values of x and y is: {}, {}", x, y); // error
}
