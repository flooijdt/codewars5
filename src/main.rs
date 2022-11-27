fn main() {
    println!("Hello, world!");
    rgb(422, 42, 42);
}

fn rgb(r: i32, g: i32, b: i32) -> String {
    format!("{r:X?}");
    println!("{:?}", r);

    if r <= 255 && r >= 0 {
        let r: String = format!("{r:X?}");
    } else {
        if r > 255 {
            let r = String::from("FF");
        } else {
            let r = String::from("00");
        }
    }

    if g <= 255 && g >= 0 {
        let g: String = format!("{g:X?}");
    } else {
        if g > 255 {
            let g = String::from("FF");
        } else {
            let g = String::from("00");
        }
    }

    if b <= 255 && b >= 0 {
        let b: String = format!("{b:X?}");
    } else {
        if b > 255 {
            let b = String::from("FF");
        } else {
            let b = String::from("00");
        }
    }

    if g <= 255 && g >= 0 {
        let g: String = format!("{g:X?}");
    }

    if b <= 255 && b >= 0 {
        let b: String = format!("{b:X?}");
    }

    if g >= 255 {
        let g: &str = "FF";
    }

    if b >= 255 {
        let b: &str = "FF";
    }

    if r <= 0 {
        let r: &str = "00";
    }

    if g <= 0 {
        let g: &str = "00";
    }

    if b <= 0 {
        let b: &str = "00";
    }

    String::from(r).push_str(String::from(g));

    String::from("whar")
}
