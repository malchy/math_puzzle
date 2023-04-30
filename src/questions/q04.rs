
fn cutbar(m:i32, n:i32, current:i32) -> i32{
    if current >= n {
        // 切り終えた
        0
    }else if current < m{
        // 次は現在の2倍
        1 + cutbar(m, n, current * 2)
    }else{
        1 + cutbar(m, n, current+m)
    }
}

pub fn q04_1(){
    let result = cutbar(3, 20, 1);
    println!("{}", result);
    let result2 = cutbar(5, 100, 1);
    println!("{}", result2);
}

fn cutbar2(m:i32, n:i32) -> i32{
    let mut count = 0;
    let mut current = 1;
    while n > current {
        current = if current < m{
            current + current
        }else{
            current + m
        };
        count += 1;
    }
    count
}

pub fn q04_2(){
    let result = cutbar2(3, 20);
    println!("{}", result);
    let result2 = cutbar2(5, 100);
    println!("{}", result2);
}
