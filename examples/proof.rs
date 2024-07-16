use trust_me_2::trust_me;

static mut UNSAFE_ACCESS: i32 = 0;
#[trust_me]
fn secretly_unsafe() {
    UNSAFE_ACCESS += 1;
}

fn main() {
    unsafe {
        println!("Original value: {:#?}", UNSAFE_ACCESS);
    }
    secretly_unsafe();
    unsafe {
        println!("Modified value: {:#?}", UNSAFE_ACCESS);
    }
}
