use librtdbg::register_fns;
use rhai::Engine;

pub fn setup_engine(engine: &mut Engine) {
    register_fns!(engine, {
        "read_mem" => read_mem,
        "write_mem_arr" => write_mem_arr,
        "write_mem_string" => write_mem_string
    });
}

// Reading a specific amount of data from an address into an array
fn read_mem(addy: i64, size: i64) -> Vec<u8> {
    let addy = addy as *const u8;
    let size = size as usize;

    let mut output = Vec::new();

    for i in 0..size {
        unsafe {
            output.push(*(addy.add(i)));
        }
    }

    output
}

// Writing an array to a specific address
fn write_mem_arr(addy: i64, new: Vec<u8>) {
    let addy = addy as *mut u8;

    for (i, byte) in new.iter().enumerate() {
        unsafe {
            *(addy.add(i)) = *byte;
        }
    }
}

// Writing a string to a specific address
fn write_mem_string(addy: i64, new: String) {
    let addy = addy;
    let new = new.as_bytes();

    write_mem_arr(addy, new.into());
}
