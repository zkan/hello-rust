mod my_func;

use my_func::my_func;
use my_func::MyStruct;

mod my_utils;

use my_utils::greeting::hey;

fn main() {
    my_func();

    let my_struct = MyStruct {
        name: "John".to_string(),
    };
    println!("{}", my_struct.name);

    hey();
}
