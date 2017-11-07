/*
Если хочется референс на поле структуры, лежащей за RefCell


struct Foo{
    field: u32
}


fn foo(v: Rc<RefCell<Foo>>) -> &u32{

}


проблема заключается в том, что borrow живет только внутри структуры, поэтому, так не получится.
Для ссылок на внутрености RefCell есть Ref, и работает это как-то так:

*/

use std::cell::{Ref, RefCell};
use std::rc::Rc;

struct StructB{
    some_b_field: u32
}

struct StructA {
    field: Rc<RefCell<StructB>>,
}

impl StructA{

    fn foo<'a>(&'a self)->Ref<'a, u32>{
        Ref::map( self.field.borrow(), |b| &b.some_b_field)
    }

}


fn main(){
}