fn main() {
    let mut i: i32 = 1;
    while i < 100 {
        
        if i % 5 == 0 && i % 3 == 0 {
            println!("fizbuz");
        }else if i % 3 == 0 {
            println!("buz");

        }else if i % 5 == 0  {
            println!("fiz");
        }else {
            println!("{}",i);
        }
        
        i = i + 1;
    }
}
