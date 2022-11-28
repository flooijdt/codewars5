fn main() {
    println!("{}", rgb(4, 0, 257));
}

fn rgb(mut r: i32, mut g: i32, mut b: i32) -> String {
    format!("{r:2X?}");
    println!("{:02X}", r);

    if (0..=255).contains(&r) {
        let r: String = format!("{r:02X?}");
    } else {
        if r > 255 {
            r = 255;
        } else {
            r = 0;
        }
    }

    if (0..=255).contains(&g) {
        let g: String = format!("{g:02X?}");
    } else {
        if g > 255 {
            g = 255;
        } else {
            g = 0;
        }
    }

    if (0..=255).contains(&b) {
        let b: String = format!("{b:02X?}");
    } else {
        if b > 255 {
            b = 255;
            println!("FFCACTA");
        } else {
            b = 0;
        }
    }
    println!("{:02X}{:02X}{:02X}", r, g, b);
    format!("{r:02X}{g:02X}{b:02X}")
}
