fn a() {
    b();
}

fn b() {
    c(23);
}

fn c(x:i32) {
    if x == 23{
        panic!("Panic ....")
    }
}

fn main() {
    a();
}
