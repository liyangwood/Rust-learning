use std::env;

fn main() {
    let current_dir = env::current_dir().expect("invalid path");
    println!("The current directory is {}", current_dir.display());

    // for (key, value) in env::vars_os() {
    //     println!("{:?}: {:?}", key, value);
    // }

    match env::current_exe() {
        Ok(exe_path) => println!("Path of this executable is: {}", exe_path.display()),
        Err(e) => println!("failed to get current exe path: {}", e),
    };
}
