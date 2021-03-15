use std::{
    fmt::Display,
    rc::Rc,
    sync::{mpsc::channel, Arc, Mutex},
    thread,
};

pub fn main() {
    //If it ends in a ! it's a macro such as println!
    println!("Hello, world!");
    single_owner();
    thread_safety();
    control_flow();
}

//FYI no function overloading
// Generic function accepts any type that implements the display trait
pub fn print_generic<T: Display>(num: T) {
    println!("number: {}", num);
}

fn print_u16(num: u16) {
    println!("number: {}", num);
}

fn print_string(num: String) {
    println!("number: {}", num);
}

fn print_borrowed_string(num: &String) {
    println!("number: {}", num);
}

///Rust only allows values to have one owner, although copyable types sometimes hide the complexity and can confuse
///newcomers. Values are also move by default, not copy by default which is unusual.
fn single_owner() {
    let owned_string = String::from("Hello");
    print_string(owned_string);
    //Compiler error
    // print_string(owned_string);

    //Variable shadowing, due to strict ownership rules the compiler is fine with you re-using names
    let owned_string = String::from("Hello");
    // This is okay as the string is borrowed by the function with ownership being returned
    print_borrowed_string(&owned_string);
    print_borrowed_string(&owned_string);
    let x = 42;
    //Okay
    print_u16(x);
    //Still okay as most primitives implement copy.
    print_u16(x);

    //Generics
    print_generic(x);
    print_generic(&owned_string);
    print_generic(owned_string);
}

fn thread_safety() {
    //Rc won't compile as it is not thread safe
    let data = Arc::new(Mutex::new(0));
    // let data = Arc::new(0);
    // let data = Rc::new(Mutex::new(0));

    let (tx, rx) = channel();
    for _ in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());
        thread::spawn(move || {
            // The shared state can only be accessed once the lock is held.
            // Our non-atomic increment is safe because we're the only thread
            // which can access the shared state when the lock is held.
            //
            // We unwrap() the return value to assert that we are not expecting
            // threads to ever fail while holding the lock.
            let mut data = data.lock().unwrap();
            // let mut data = data.lock().unwrap();
            *data += 1;
            if *data == 10 {
                tx.send(()).unwrap();
            }
            // the lock is unlocked here when `data` goes out of scope.
        });
    }

    rx.recv().unwrap();
}

fn control_flow() {
    match alex_get_first_or_a_default(vec![]) {
        Ok(_) => {}
        Err(e) => {
            println!("Alex's function didn't work again, {}", e)
        }
    }

    match alex_get_first_or_a_default(vec![4, 3, 2, 1]) {
        Ok(x) => {
            println!("Managed to get the first value {}", x)
        }
        Err(e) => {
            println!("Alex's function didn't work again, {}", e)
        }
    }
}

fn alex_get_first_or_a_default(vec: Vec<u32>) -> Result<u32> {
    //The "?" immediately exits the function with the result if it is an error
    Ok(vec.first().ok_or(RazorSecureError)?.clone())
}

type Result<T> = std::result::Result<T, RazorSecureError>;

//Special unit struct only used for control flow purposes
#[derive(Debug, Clone)]
struct RazorSecureError;

impl Display for RazorSecureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "it must be Lewis' fault")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_test() {
        assert!(true);
    }
}
