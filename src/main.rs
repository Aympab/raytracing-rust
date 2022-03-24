mod color;

fn main() {
    println!("Hello world !");
    let blue = color::RGBColor { r:0, g:0, b:255};

    println!("{}, {}, {}", blue.r, blue.g, blue.b);
}

#[test]
fn should_pass(){
    assert!(1 == 1);
}

#[test]
fn should_fail(){
    unimplemented!();
}