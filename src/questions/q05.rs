

/// 素朴なやり方
pub fn q05_1(){

    let mut cnt = 0;
    for i in 0..3 {  // 500円玉最大2枚
        for j in 0..11{ // 100円玉最大10枚
            for k in 0..16{ // 50円玉最大15枚 (コインの合計枚数)
                for l in 0..16{ // 10円玉最大15枚 (コインの合計枚数)
                    if i + j + k + l <= 15{
                        if 500*i + 100*j + 50*k + 10*l == 1000{
                            cnt += 1;
                        }
                    }
                }
            }
        }
    }
    println!("count = {}",cnt);
}

use itertools::Itertools;
/// イテレータ使用
pub fn q05_2(){
    let mut cnt = 0;
    let coins = vec![10, 50, 100, 500];
    for i in 0..16{
        for x in coins.clone().into_iter().combinations_with_replacement(i){
            let sum: i32= x.into_iter().sum();
            if sum == 1000{
                //println!("{:?}", x);
                cnt += 1;
            }
        }
    }
    println!("count = {}",cnt);
}

/// 再帰使用
pub fn q05_3(){
    let mut cnt = 0;
    let mut coins = vec![500, 100, 50, 10];
    change(&mut cnt, 1000, &mut coins, 15);

    println!("count = {}", cnt);
}

fn change(cnt: &mut i32, target: i32, coins: &mut Vec<i32>, usable: i32){
    let coin = *coins.clone().first().unwrap();
    coins.remove(0);
    let rest : i32 = target / coin;
    if coins.len() == 0 {
        if rest <= usable{
            *cnt+=1;
        }
    }else{
        for i in 0..=rest{
            change(cnt, target - coin*i, &mut coins.clone(), usable - i);
        }
    }
}


