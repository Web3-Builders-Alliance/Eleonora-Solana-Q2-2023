pub fn run(){
    //Default is "i32"
    let x = 1;

    //Default is "i64"
    let y = 2.5;

    //Add explixt type
    let z: i64 = 4545445454545;

    //Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i32: {}", std::i64::MAX);

    //Boolean
    let is_active: bool = true;

    //Get boolena from expression
    let is_greater: bool = 10 < 5;

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}