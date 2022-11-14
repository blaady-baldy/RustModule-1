// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn main() {
    first_name();
    last_name();
}

fn first_name() {
    println!("Devansh");
}

fn last_name() -> (){
    let name = String::from("Singh");
    println!("{:?}", name);
}


