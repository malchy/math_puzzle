use chrono::prelude::*;

pub fn q07_1(){
    let start = NaiveDate::from_ymd_opt(1964, 10, 10).unwrap();
    let end = NaiveDate::from_ymd_opt(2020, 7, 24).unwrap();
    println!("from {} to {}", start, end);
    let mut cnt = 0;
    // startからendまで繰り返す
    for day in start.iter_days().take_while(|x| x<= &end){
        // 日付を8桁の10進数で表現
        let dti :i64 = format!("{:04}{:02}{:02}",day.year(),day.month(),day.day()).parse().unwrap();
        // 逆順の2進数文字列に変換
        let cat = format!("{:b}", dti).chars().rev().collect::<String>();
        // 2進数→10進数に変換
        let rev_dti :i64 = i64::from_str_radix(cat.as_str(), 2).unwrap();

        if dti == rev_dti {
            println!("{}", day);
            cnt += 1;
        }

    }
    println!("There are {} days", cnt);
}


pub fn q07_2(){
    // 対象の期間で2進数の5文字目から8文字を抽出 (最初の4文字は全て"1001")
    let mut cnt = 0;
    let from_left = i32::from_str_radix(&format!("{:b}", 19641010)[4..12], 2).unwrap() ;
    let to_left   = i32::from_str_radix(&format!("{:b}", 20200724)[4..12], 2).unwrap() ;
    //println!("{} to {}", from_left, to_left);
    // 左右の8文字をループ
    for i in from_left..=to_left{
        let l = format!("{:08b}",i);
        let r = l.chars().rev().collect::<String>();

        for m in 0..=1{
            let values = format!("1001{}{}{}1001", l, m, r);
            let value2 = i32::from_str_radix( &values, 2).unwrap();
            // 日付として成立している場合のみ表示
            let day = NaiveDate::parse_from_str(&format!("{}", value2), "%Y%m%d") ;
            match day {
                Ok(v) => {
                    println!("{}", v);
                    cnt += 1;
                },
                Err(_) => continue,
            }
        }
    }
    println!("There are {} days", cnt);
}