fn main() {
    let mut res = 42;
    let option = Some(12);
    // Use if let instead of for loop for Option.
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}