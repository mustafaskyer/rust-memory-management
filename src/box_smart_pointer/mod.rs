#[derive(Debug)]
pub enum List {
    Cons(i32, Option<Box<List>>),
}

#[derive(Debug, Clone, Copy)]
pub struct Huge_data;

#[derive(Debug, Clone, Copy)]
pub struct Small_Data;

pub trait Storage {}

impl Storage for Huge_data {}
impl Storage for Small_Data {}

#[derive(Debug)]
pub enum ListNode<T> {
    Node(T, Box<ListNode<T>>),
    None,
}

#[derive(Debug)]
pub struct AudioSample;

#[derive(Debug)]
pub struct ImageFile;

pub trait Media {}

impl Media for AudioSample {}
impl Media for ImageFile {}
