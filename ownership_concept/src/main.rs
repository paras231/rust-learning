fn main() {
    
    // let mut s = String::from("Hello");
    // s.push_str(", world!");
    // println!("{s}");
    // let s1 = String::from("Hello"); 
    // let s2 = s1;  // will give error as s1 is moved to s2  (because this was stored in heap not in stack)
    // println!("{s2}");

    // let s = String::from("Hello");
    // takes_ownership(s);

    // let x = 5;
    // // makes_copy(x);

    // Structs

    struct User {
    active: bool,
    username: String,
    email: String,
}

    let mut user1 = User{
        active : true,
        username : String::from("Paras"),
        email : String::from("exampleuser@gmail.com"),
    };
    user1.username = String::from("Sharma"); 
    println!("{user1}");
}



// fn takes_ownership(s:String){
//     println!("{s}");
// }

// fn makes_copy(x:i32){
//     println!("{x}");
// }