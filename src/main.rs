fn main() {
    println!("Hello, world!");
    rgb(422, 42, 42);
}

fn rgb(r: i32, g: i32, b: i32) -> String {
    println!("{:?}", format!("{r:X?}"));

    if r >= 255 {
        r = "FF";
    }

    if g >= 255 {
        g = "FF";
    }

    if b >= 255 {
        b = "FF";
    }

    if r <= 0 {
        r = "00";
    }

    if g <= 0 {
        g = "00";
    }

    if b <= 0 {
        b = "00";
    }

    String::from("tifo")
}
