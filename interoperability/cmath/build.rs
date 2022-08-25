use cc::Build;

fn main() {
    Build::new().file("libc/math.c").compile("math");
}
