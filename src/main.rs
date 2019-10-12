mod idkhan {
    pub mod sultan{
        pub fn id1(){
            println!("ID1");
        }
    }
}

mod basit{
    pub fn b1(){
        println!("b1");
    }

}

mod shahana {
    pub fn anum(){
        println!("Anum");
    }
}

fn main() {
    println!("Hello, world!");
    shahana::anum();
    idkhan::sultan::id1();    
}
