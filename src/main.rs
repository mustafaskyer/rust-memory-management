mod generic_static;
mod specifier;
use generic_static::pick_num;
use specifier::pick_num as pick;

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
}
