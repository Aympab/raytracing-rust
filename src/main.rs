fn main() {
    println!("Hello world !");
}

#[test]
fn should_pass(){
    assert!(1 == 1);
}

#[test]
fn should_fail(){
    unimplemented!();
}

