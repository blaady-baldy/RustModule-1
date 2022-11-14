
fn main() {
    let boolean = true;

    // Fill the blank with a match expression:
    //
    // boolean = true => binary = 1
    // boolean = false =>  binary = 0
    let mut val = 0;

    match boolean {
        true => val = 1,
        false => val = 0,
    };

    let binary = val;
    

    assert_eq!(binary, 1);

    println!("Success!");
}
