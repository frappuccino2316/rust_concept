fn main() {
    let color = ColorRGB(255, 128, 0);

    println!("{}, {}, {}", color.0, color.1, color.2);
}

struct ColorRGB(u32, u32, u32);
