use std::rc::Rc;
use std::cell::RefCell;

pub trait Material {
    fn bind(&self);
}

pub type BoxMat = Rc<RefCell<Material>>;