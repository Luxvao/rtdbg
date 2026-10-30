use std::time::Duration;

fn main() {
    let output_string = "Hello World!";

    println!("Output string pointer: {output_string:p}");

    loop {
        println!("{output_string}");

        std::thread::sleep(Duration::from_secs(1));
    }
}
