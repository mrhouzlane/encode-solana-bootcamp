pub fn run(){
    solana101("Ethereum", "Solana");

    // Bind function values to variables
    let get_sum = add(10,11);
    println!("Sum: {}", get_sum);

    // Closure 
    let add_nums = |n1: i32, n2: i32| n1 + n2; 
    println!("C Sum: {}", add_nums(3,3));

}

fn solana101(network: &str, othernetwork: &str){
    println!("{} is bigger ecosystem than {}", network, othernetwork);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2 
}