use std::cell::{RefCell, RefMut};
use crate::prelude::WaylandSockMsg;

pub struct WaylandObject<T: WaylandObjectImpl + ?Sized>{
    obj: Box<RefCell<T>>
}

pub struct WaylandObjectRef<'a, T: WaylandObjectImpl + ?Sized>{
    obj_ref: RefMut<'a, T>
}

pub trait WaylandObjectImpl{
    fn get_id(&self) -> u32;
    fn get_type(&self) -> String;
    fn borrow_mut(&mut self) -> WaylandObjectRef<dyn WaylandObjectImpl>;
    fn borrow_children(&mut self) -> Vec<WaylandObjectRef<dyn WaylandObjectImpl>>;
    fn msg_downstream(&self, msg: WaylandSockMsg);
    fn msg_upstream(&mut self, msg: WaylandSockMsg);
}

impl<T: WaylandObjectImpl> WaylandObject<T> {
    pub fn from<A: WaylandObjectImpl>(obj: A) -> WaylandObject<A>{
        WaylandObject{
            obj: Box::new(RefCell::new(obj))
        }
    }
}

impl<T: WaylandObjectImpl + ?Sized> WaylandObjectImpl for WaylandObject<T> {
    fn get_id(&self) -> u32 {
        self.obj.borrow().get_id()
    }

    fn get_type(&self) -> String {
        self.obj.borrow().get_type()
    }

    fn borrow_mut(&mut self) -> WaylandObjectRef<dyn WaylandObjectImpl> {
        self.obj.get_mut().borrow_mut()
    }

    fn borrow_children(&mut self) -> Vec<WaylandObjectRef<dyn WaylandObjectImpl>> {
        self.obj.get_mut().borrow_children()
    }

    fn msg_downstream(&self, msg: WaylandSockMsg){
        self.obj.borrow_mut().msg_downstream(msg);
    }

    fn msg_upstream(&mut self, msg: WaylandSockMsg){
        self.obj.borrow_mut().msg_upstream(msg);
    }
}

impl WaylandObjectImpl for WaylandObjectRef<'_, dyn WaylandObjectImpl> {
    fn get_id(&self) -> u32 {
        todo!()
    }

    fn get_type(&self) -> String {
        todo!()
    }

    fn borrow_mut<'a>(&mut self) -> WaylandObjectRef<dyn WaylandObjectImpl> {
        todo!()
    }

    fn borrow_children(&mut self) -> Vec<WaylandObjectRef<dyn WaylandObjectImpl>> {
        todo!()
    }

    fn msg_downstream(&self, _msg: WaylandSockMsg) {
        todo!()
    }

    fn msg_upstream(&mut self, _msg: WaylandSockMsg) {
        todo!()
    }
}