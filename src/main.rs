

fn main(){
    let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let iter = v1.iter();
    let iter2 = iter.filter(|x| *x % 2 == 0);
    for i in iter2 {
        println!("{}", i);
    }
}



//--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// fn main(){

//     let mut vec = Vec::new();
//     //We have defined an Array
//     vec.push(1);
//     vec.push(2);
//     vec.push(3);
//     vec.push(4);
//     vec.push(5);
//     vec.push(6);
//     vec.push(7);
//     vec.push(8);

//     let new_vec = even_filter(&vec);
//     // println!("{:?}", vec); // [1, 2, 3]
//     println!("{:?}", new_vec); // [1, 2, 3]
// }

// fn even_filter(vec : &Vec<i32>) -> Vec<i32> {
//     let mut new_vec: Vec<i32> = Vec::new();

//     for i in vec {
//         if i % 2 == 0 {
//             new_vec.push(*i); // *i is used to dereference the value of i
//         }
//     }
//     new_vec
// }

//--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// struct User {
//     name: String,
//     age : u32,
//     active: bool
// }
// fn main() {
//     let name = String::from("John Doe");
//     let user = User {
//         name,
//         age: 25,
//         active: true
//     };
//     print!("User: {} is {} years old and is active: {}", user.name, user.age, user.active);
// }
