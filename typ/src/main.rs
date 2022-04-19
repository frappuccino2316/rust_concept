const BUFSIZE: usize = 1024;
static OFFSET: usize = 15;

fn main() {
    let offset_ref = &OFFSET;

    println!("bufsize = {}", BUFSIZE);
    println!("offset = {}", offset_ref);

    add_static();
    add_static();
    add_static();
}

fn add_static() {
    const INCREMENT: usize = 2;
    static mut ADD: usize = 1;

    unsafe {
        ADD += INCREMENT;
        println!("add = {}", ADD);
    }
}
