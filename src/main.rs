fn main() {
    println!("Hello, world!");
    rgb(422, 42, 42);
}

fn rgb(r: i32, g: i32, b: i32) -> String {
    format!("{r:X?}");
    println!("{:?}", r);

    // if r >= 255 {
    //     let r: &str = "FF";
    // }

    // if g >= 255 {
    //     let g: &str = "FF";
    // }

    // if b >= 255 {
    //     let b: &str = "FF";
    // }

    // if r <= 0 {
    //     let r: &str = "00";
    // }

    // if g <= 0 {
    //     let g: &str = "00";
    // }

    // if b <= 0 {
    //     let b: &str = "00";
    // }

    // String::from(r).push_str(String::from(g));

    String::from("whar")
}
