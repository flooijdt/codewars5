fn main() {
    println!("{}", rgb(4, 0, 255));
}

fn rgb(r: i32, g: i32, b: i32) -> String {
    // format!("{r:2X?}");
    //println!("{:02X}", r);

    if r <= 255 && r >= 0 {
        let r: String = format!("{r:02X?}");
    } else {
        if r > 255 {
            let r = String::from("FF");
        } else {
            let r = String::from("00");
        }
    }

    if g <= 255 && g >= 0 {
        let g: String = format!("{g:02X?}");
    } else {
        if g > 255 {
            let g = String::from("FF");
        } else {
            let g = String::from("00");
        }
    }

    if b <= 255 && b >= 0 {
        let b: String = format!("{b:02X?}");
    } else {
        if b > 255 {
            let b = String::from("FF");
        } else {
            let b = String::from("00");
        }
    }
    println!("{:2X}{:2X}{:2X}", r, g, b);
    format!("{r}{g}{b}")
}
