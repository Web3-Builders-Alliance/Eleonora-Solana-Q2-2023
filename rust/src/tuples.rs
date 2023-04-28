pub fn run(){
    let person: (&str, &str, i8) = ("Brad", "Mass", 37);

    println!("{} in from {} and is {}", person.0, person.1, person.2);
}