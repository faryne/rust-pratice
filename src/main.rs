fn main() {
    // let mut a = 1;
    // let b = {
    //     a = 5;
    //     a + 1
    // };
    //
    // (e, f) = d();
    // println!("value of e: {}, value of f: {}", e, f);
    //
    // println!("{}", a);
    // println!("{}", b)

    let s1 = String::from("hello");
    let s2 = s1.clone();

    let s3: &str = "Hello World";
    let s4 = s3;

    println!("s1 = {}, s2 = {}", s1, s2);
    println!("s3 = {}, s4 = {}", s3, s4);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

// fn d() -> (i32, i32) {
//     let (a, b) = (1, 2);
//     (a, b)
// }
