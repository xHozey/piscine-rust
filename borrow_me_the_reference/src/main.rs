use borrow_me_the_reference::*;

fn main() {
    let mut a = "hef--llo".to_owned();


    delete_and_backspace(&mut a);

    println!("{:?}", a);
}