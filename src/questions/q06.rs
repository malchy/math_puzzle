

pub fn q06(){
    let mut cnt = 0;
    for i in (2..=10000).step_by(2){
        // 初回は必ずn*3+1
        let mut n = 3 * i + 1;
        while n != 1{
            if n %2 == 1{
                // 奇数の場合3倍して1加算
                n = n *3 + 1;
            }else{
                // 偶数の場合2で割る
                n = n / 2;
            }
            if n == i{
                // 元の数字に戻った
                //println!("{}",n);
                cnt += 1 ;
                continue;
            }
        }
    }
    println!("{}", cnt);
}