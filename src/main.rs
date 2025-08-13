mod elision;
mod generic_static;
mod specifier;
mod structs;
use generic_static::pick_num;
use specifier::pick_num as pick;

use elision::{return_str, return_str_with_multi};
use structs::ArrayProcessor;

fn main() {
    let num1 = 5;
    let picked_value;
    {
        let num2 = 10;
        picked_value = pick_num(&num1, &num2);
    }
    let num3 = 7;
    let picked = pick(&num1, &num3);
    println!("{picked_value}");
    println!("Picked: {picked}");

    // ** Elision **
    let str1 = "some string ...";
    let str2 = "some string two ...";
    let str1_return_output = return_str(str1);
    let str1_return_with_multi_output = return_str_with_multi(&str1, &str2);

    println!("{str1_return_output}");
    println!("With multi {str1_return_with_multi_output}");
    // ** Elision **

    // ** Structs ** //
    let mut some_data = ArrayProcessor { data: &[1, 2, 3] };
    println!("Array Processor: {:?}", some_data.data);
    let new_data = [4, 5, 6];
    some_data.update_data(&new_data);
    println!("Array Processor: {:?}", some_data.data);
    // ** Structs ** //
}
