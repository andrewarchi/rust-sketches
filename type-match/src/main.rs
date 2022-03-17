#![feature(const_type_id, downcast_unchecked, inline_const, inline_const_pat)]

use std::any::{Any, TypeId};

fn main() {
    match_type(&A("Hello".to_string()));
    match_type(&B(42));
    match_type(&C(false));
}

struct A(String);
struct B(i32);
struct C(bool);

fn match_type(v: &dyn Any) {
    match v.type_id() {
        const { TypeId::of::<A>() } => {
            // SAFETY: type already checked
            let A(a) = unsafe { v.downcast_ref_unchecked::<A>() };
            println!("A({:?})", a);
        }
        const { TypeId::of::<B>() } => {
            // SAFETY: type already checked
            let B(b) = unsafe { v.downcast_ref_unchecked::<B>() };
            println!("B({:?})", b);
        }
        _ => println!("other"),
    }

    // Want a macro like this:
    /*
    match_type!(v, {
        A(a) => println!("A({:?})", a),
        B(b) => println!("B({:?})", b),
        _ => println!("other"),
    })
    */
}
