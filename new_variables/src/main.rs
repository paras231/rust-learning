// fn main() {
//     // let x = 5;
//     // x = 20;  // will throw an error as variables are immutable by defaults
//     // println!("{x}");

//     // let mut x = 5;
//     // x = 20; // works fine 
//     // println!("{x}");

//     //  Constants
//     // const MIN_HEIGHT: u32 = 30;
//     // println!("min height is : {MIN_HEIGHT}");

//     //  Shadowing Variables

//     // let x  = 5;
//     // let x  = x + 1;

//     // {
//     //     let x = x * 2;
//     //     println!("The value of x in the inner scope is: {x}");
//     // }
//     // println!("The value of x in outer scrope {x}");

//     // let y = 34.60;  // float integer

//     // let x = 20;

//     // value_getter(x);
//     // let value = five();
//     // println!("value is : {value}");
// }


// fn value_getter(x : i32){
//     println!("The value of x is {x}");
// }


// fn five() -> i32{
//     return 5 ;
// }


//  Working with loops
fn main(){
  let mut counter = 0;
  let result = loop{
      counter += 1;
      println!("counter increasing :{counter}");
      if counter == 10{
         break counter * 2;
      }
  };
  println!("The result is {result}");
}