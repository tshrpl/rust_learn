

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug)]
enum List {
	// Cons(i32, Rc<List>),
	Cons(Rc<RefCell<i32>>, Rc<List>),
	Nil
}


fn main() {
	// let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
	// println!("count after creating a = {}", Rc::strong_count(&a));
	// let b = Cons(3, Rc::clone(&a));
	// println!("count after creating b = {}", Rc::strong_count(&a));
	// {
	// 	let c = Cons(4, Rc::clone(&a));
	// 	println!("count after creating c = {}", Rc::strong_count(&a));
	// }
	// println!("count after c goes out of scope = {}", Rc::strong_count(&a));


	let value = Rc::new(RefCell::new(5));

	let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

	let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
	let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

	*value.borrow_mut() += 10;

	println!("a after = {:?}", a);
	println!("b after = {:?}", b);
	println!("c after = {:?}", c);
}

