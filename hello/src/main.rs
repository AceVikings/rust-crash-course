fn main() {
    let val: u8 = 48;
    let another_val: i8 = 48;
    println!("Hello, world! {} {}",val,another_val);
    
    let name: String = String::from("Hello-there");
    println!("{}",name);

    let slice = &name[0 .. 1];
    println!("{}",slice);

    conditional(5);

    for_loop(10);

    let name = String::from("Birb");
    let birb = Bird {name,attack:5};
    birb.print_name();
}

pub fn is_even(num: u8) -> bool {
     let even: u8 = num%2;
     even == 0
}

pub fn conditional(num:i32) {
    
    if num > 0 {
        println!("greater than 0");
    }
    else if num < 0{
        println!("less than 0");
    }
    else{
        println!("is 0");
    }
}

pub fn for_loop(num:u8){
    for i in 0..num {
        println!("{}",i);
    }
}
struct Bird {
    name: String,
    attack: u64
}

impl Bird{
    fn print_name(&self){
        println!("{}",self.name);
    }
}

impl Animal for Bird{
    fn can_fly(&self) -> bool {
        true
    }
}

trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool{
        true
    }
}

pub fn vector(){
    let mut vec: Vec<u64> = vec![1,2,3,4,5];
    println!("{}",vec.len());
}