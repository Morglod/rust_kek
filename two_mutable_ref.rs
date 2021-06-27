struct S (i32);

impl S {
    fn match_do(&mut self) -> bool {
        (self.0) += 1;
        println!("hello Rust world {}", self.0);
        false
    }
}

trait Kekw<T> {
    fn dat_rust<'a>(self) -> &'a mut T;
}

impl<T> Kekw<T> for &&mut T {
    fn dat_rust<'a>(self) -> &'a mut T {
        // or with #![allow(mutable_transmutes)]
        // unsafe {
        //     return std::mem::transmute::<&&mut S, &mut &mut S>(x).match_do();
        // }
        
        // 'unsafe' is dereferencing pointer that I get from reference !!in same line!!, not doing mess with references
        // oh my legs, plz no ohh everyting is writed on C ohh
        // but no pointers plzzz
        
        // in language that slows developing & compilation in 100 times coz of borrow checker
        // you could do this MMMMMMMM
        unsafe {
            &mut *(((&(**self)) as *const T) as *mut T)
        }
    }
}

fn main() {
    let mut v = vec![S(0), S(2), S(2), S(6)];
    
    let _r = v.iter_mut()
    .find(|x| {
        x.dat_rust().match_do()
    });
}
