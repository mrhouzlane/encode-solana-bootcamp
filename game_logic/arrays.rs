use std::mem;

pub fn run(){

    let mut numbers: [i32; 4] = [1,2,3,4];

    // Re-assign value 
    numbers[2] = 20;

    println!("{:?}", numbers);

    //Get single value 
    println!("Single Value : {}", numbers[1]);

    //Get length
    println!("Length: {}", numbers.len());

    // Array are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice 
    let slice: &[i32] = &numbers[1..3] ;
    println!("Slice: {:?}", slice);

}

