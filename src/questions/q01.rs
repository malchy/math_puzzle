pub fn q01(){
    //println!("Hello, world!");
    // :x 16進数
    // :o 8進数
    // :b 2進数
    let mut i=11;
    loop{
        if format!("{:b}",i) == format!("{:b}",i).chars().rev().collect::<String>() &&
        format!("{:o}",i) == format!("{:o}",i).chars().rev().collect::<String>() &&
        format!("{}",i) == format!("{}",i).chars().rev().collect::<String>()
        {
            println!("answer: {}", i);
            println!("0b{:b}",i);
            println!("0x{:x}",i);
            break;
        }
        if i == 100{
            println!("not found");
        }
        i += 2 ;
    }
}
