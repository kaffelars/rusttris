fn main() {
    println!("grellefot");

    let mut ost = testo::Gaffel::new(3,5);

    loop {
        //
        ost.dubble();
        println!("jeb");
        println!("{}", ost.a);
    }
}



pub mod testo {
    pub struct Gaffel {
        pub a: i32,
        b: i32,
        c: i32
    }

    impl Gaffel{
        pub fn new(aa:i32, bb:i32) -> Gaffel {
            Gaffel{a:aa,b:bb,c:32}
        }

        pub fn dubble(&mut self) {
            self.a +=1;
        }
    }
}
    
