// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    if maybe_number.is_some() {
      println!("printing: {}", maybe_number.unwrap());
    } else {
      println!("None");
    }
}

fn main() {
    print_number(Some(13));
    print_number(Some(99));
    print_number(None);

    let numbers: [Option<u16>; 5] = [Some(1), Some(2), Some(3), Some(4), None];
    for &iter in numbers.iter() {
        print_number(iter);
    }
}
