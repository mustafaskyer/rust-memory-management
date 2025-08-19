use std::rc::Rc;
pub enum RcList {
    Cons(i32, Option<Rc<RcList>>),
}
