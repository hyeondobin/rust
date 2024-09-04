// fn main() {
//     let mut v = vec![1, 2, 3];
//
//     v.push(9);
//
//     let third: &i32 = &v[2];
//     println!("The third element is {third}");
//
//     let third: Option<&i32> = v.get(2);
//     match third {
//         Some(third) => println!("The third element is {third}"),
//         None => println!("There is no third element."),
//     }
//
//     let mut v = vec![100, 32, 57];
//     for i in &mut v {
//         *i += 50;
//         println!("{i}");
//     }
// }

// fn main() {
//     let mut s1 = String::from("foo");
//     let s2 = "bar";
//     s1.push_str(s2);
//     println!("s2 is {s2}");
//     println!("");
//
//     let s1 = String::from("Hello, ");
//     let s2 = String::from("world!");
//     let s3 = s1 + &s2;
//
//     println!("{}", s3);
//     println!("");
// }

use std::collections::HashMap;

// fn main() {
//     let mut scores = HashMap::new();
//
//     scores.insert(String::from("Yellow"), 50);
//     scores.insert(String::from("Blue"), 10);
//
//     for (key, value) in &scores {
//         println!("{key}: {value}");
//     }
//
//     let field_name = String::from("Favorite color");
//     let field_value = String::from("Blue");
//
//     let mut map = HashMap::new();
//     map.insert(field_name, field_value);
//
//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 10);
//
//     scores.entry(String::from("Yellow")).or_insert(50);
//     scores.entry(String::from("Blue")).or_insert(50);
//
//     println!("{:?}", scores);
// }

fn main() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
