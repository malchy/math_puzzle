extern crate eval;
use eval::eval;

pub fn q02(){
    let ops = vec![ "*", ""]; // 高速化版：+-/を使うと4桁の整数にならない
    //let ops = vec!["+", "-", "*", "/", ""];
    for i in 1000..=10000 {
        let c = i.to_string();
        for op1  in ops.iter(){
            for op2 in ops.iter(){
                for op3 in ops.iter(){
                    let valuation = format!("{}{}{}{}{}{}{}", &c[3..4], op1, &c[2..3], op2, &c[1..2], op3, &c[0..1]);
                    // 演算子が必ず1つは必要
                    if valuation.len() > 4{
                        match eval(&valuation){
                            Ok(v) => {
                                if i == v {
                                    println!("answer: {}", valuation);
                                }
                            },
                            Err(x) => continue, // 計算式が成り立たない場合 (0徐算など)
                        }
                    }
                }
            } 
        }
    }
}