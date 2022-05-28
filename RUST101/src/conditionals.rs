pub fn run() {

    let age: u8 = 22; 
    let check_id: bool = true; 
    let knows_person_of_age: bool = true;  // Tricky 

    //If/Else 
    if age >=21 && check_id || knows_person_of_age { // tricky OR 
        println!("YWhat do you want to drink ? ");
    } else if age<21 && check_id {
        println!{"Sorry you have to leave"};
    }else {
        println!("Bartender : Show me your ID");
    }

    // Shorthand If 
    let is_of_age = if age > 21 { true } else { false };
    println!("Is of age {} ", is_of_age)

}