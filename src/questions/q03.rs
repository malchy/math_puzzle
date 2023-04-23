

pub fn q03(){
    const N:usize = 100;  // size
    let mut cards : [bool;N] = [false;N];
    for i in 2..=N{
        let mut j = i-1;
        while j < N {
            cards[j] = !cards[j];
            j += i ;
        }
    }
    for i in 0..N{
        if !cards[i]{
            println!("{}", i+1);
        }
    }
}

// 別解
pub fn q03_2(){
    for i in 1..=100 {
        let mut flag = false;
        for j in 1..=100 {
            if i % j == 0 {
                flag  = !flag;
            }
        }
        if flag {
            println!("{}", i);
        }
    }
}