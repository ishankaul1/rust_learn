// Note: Rust requires you to type label function params.
fn another_function(value: i32, unit_label: char) {
    println!("In another function!");
    println!("The 'measurement' is {value}{unit_label}.");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// fn plus_one_broken(x: i32) -> i32 {
//     x + 1;
// }

fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("Val of y: {y}");

    another_function(five(), 'h');

    let five_plus_one = plus_one(five());
    println!("Five plus one is {five_plus_one}");
}
