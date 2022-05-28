use std::mem;

pub fn run(){

    let mut numbers: Vec<i32> = vec![1,2,3,4];

    // Re-assign value 
    numbers[2] = 20;

    // Add on to vector 
    numbers.push(5); 
    numbers.push(6);

    //Pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    //Get single value 
    println!("Single Value : {}", numbers[1]);

    //Get length
    println!("Vector Length: {}", numbers.len());

    // Vector are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice 
    let slice: &[i32] = &numbers[1..3] ;
    println!("Slice: {:?}", slice);

    // Loop through vector values 
    for z in numbers.iter(){
        println!("Numbers {}", z);
    }

    // Loop & mutate values 
    for x in numbers.iter_mut(){
        *x *=2; 

    }

    println!("{:?}", numbers);

}

