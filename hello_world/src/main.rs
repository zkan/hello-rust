mod my_func;

use my_func::MyStruct;

fn main() {
    my_func::my_func();

    let my_struct = MyStruct { name: "John".to_string() };
    println!("{}", my_struct.name)
}
