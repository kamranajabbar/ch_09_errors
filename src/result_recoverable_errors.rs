use std::fs::File;

pub fn run() {
    // enum Result<T, E> { 
    //     Ok(T), 
    //     Err(E), 
    // }

    let f = File::open("hello.txt");

    let f = match f {
        Result::Ok(file) => file, 
        Result::Err(error) => { 
            panic!("Problem opening the file: {:?}", error)
        }, 
    };
}