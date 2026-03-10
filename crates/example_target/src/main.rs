use std::io::Read;

fn main() {
    let output_string = "Hello World!";

    println!("Output string pointer: {:p}", output_string);

    loop {
        let mut buf = [0; 1];

        println!("{}", output_string);
        std::io::stdin()
            .read_exact(&mut buf)
            .expect("Failed to read from stdin");
    }
}
