use std::ops::{Deref, DerefMut};

#[allow(dead_code)]
mod board;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Struct<'a, A: 'a> {
    a: &'a mut A,
}

impl<'a, A> Struct<'a, A> {
    fn a(&mut self) -> Option<&mut A> {
        Some(self.a)
    }

    fn a_ref<'b: 'a>(&'b mut self) -> Option<&'a mut A> {
        Some(self.a)
    }
}

impl<'a, A> Iterator for Struct<'a, A> {
    // impl<'a, 'b: 'a, A> Iterator for &'b Struct<'a, A> {
    // impl<'a, 'b, A> Iterator for Struct<'a, A> {
    type Item = &'a mut A;
    // type Item = &'b mut A;

    fn next(&mut self) -> Option<Self::Item> {
        // self.a() // doesn't live long enough
        // Some(&mut self.a) // doesn't live long enough
        // let a = &mut a;
        // Some(a)

        todo!()
    }
}

// struct Struct2<'a, A: 'a> {
//     a: Option<&'a mut A>,
// }

// impl<'a, A> Deref for Struct2<'a, A> {
//     type Target = A;

//     fn deref(&self) -> &Self::Target {
//         self.a.as_deref().unwrap()
//     }
// }

// impl<'a, A> DerefMut for Struct2<'a, A> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         let some = self.a.as_deref_mut().unwrap();
//         some
//     }
// }

// impl<'a, A> Iterator for Struct2<'a, A> {
//     type Item = &'a mut A;

//     // fn next(&mut self) -> Option<Self::Item> {
//     fn next(&mut self) -> Option<Self::Item> {
//         // let a = self.a;
//         self.a.as_deref_mut()
//     }
// }
