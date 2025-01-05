fn main() {
    let mut x = 5;
    {  // Create a separate scope
        let y = &mut x; 
        *y += 1;
    }
    { // Create a separate scope
        let z = &mut x; 
        *z += 1;
    }
    println!("x = {}", x);
}