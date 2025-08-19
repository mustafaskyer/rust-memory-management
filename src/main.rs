mod box_smart_pointer;
mod elision;
mod generic_static;
mod smar_pointer;
mod specifier;
mod structs;

use box_smart_pointer::{
    AudioSample, Huge_data, ImageFile, List, ListNode, Media, Small_Data, Storage,
};
use elision::{return_str, return_str_with_multi};
use generic_static::pick_num;
use specifier::pick_num as pick;
use structs::ArrayProcessor;

use smar_pointer::RcList;
use std::rc::Rc;

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

    // ** Smart Pointers
    let x = 0.666;
    let y = Box::new(x);
    let list = List::Cons(
        1,
        Some(Box::new(List::Cons(2, Some(Box::new(List::Cons(3, None)))))),
    );
    println!("List: {:?}", list);

    let data1 = Huge_data;
    let data2 = Box::new(Huge_data);

    let data3 = data1;
    let data4 = data2.clone();

    println!("Data 1: {:?}", data1);
    println!("Data 2: {:?}", data2);
    println!("Data 3: {:?}", data3);
    println!("Data 4: {:?}", data4);

    let data5 = Box::new(Small_Data);

    let all: Vec<Box<dyn Storage>> = vec![Box::new(data1), Box::new(data3), data5];

    let nodes = ListNode::Node(
        1,
        Box::new(ListNode::Node(
            2,
            Box::new(ListNode::Node(
                3,
                Box::new(ListNode::Node(4, Box::new(ListNode::None))),
            )),
        )),
    );
    println!("{:?}", nodes);

    let audio_1 = AudioSample;
    let audio_2 = Box::new(AudioSample);

    let audio_3 = audio_1;
    let audio_4 = audio_2;

    let image_1 = Box::new(ImageFile);

    let media_collection: Vec<Box<dyn Media>> = vec![Box::new(audio_3), audio_4, image_1];
    // ** Smart Pointers

    // ** RC . Smart Pointers
    let a = Rc::new(RcList::Cons(1, Some(Rc::new(RcList::Cons(2, None)))));
    println!("Ref count after a: {}", Rc::strong_count(&a));
    {
        let b = RcList::Cons(3, Some(Rc::clone(&a)));
        println!("Ref count after b: {}", Rc::strong_count(&a));
        let c = RcList::Cons(4, Some(Rc::clone(&a)));
        println!("Ref count after c: {}", Rc::strong_count(&a));
    }
    println!("Ref count after all: {}", Rc::strong_count(&a));
    // ** RC . Smart Pointers
}
