

pub fn q09_01(){
    // 男女の合計数(0人から数えるため+1)
    let boy: usize = 20 + 1;
    let girl: usize = 10 + 1;
    let size: usize = (boy*girl) as usize;
    let mut ary = vec![0;size];

    ary[0] = 1 ;

    for g in 0..girl{
        for b in 0..boy{
            if b != g && (boy -b ) != (girl - g){
                let tpos: usize = b + boy*g ;
                if b > 0 {
                    let lpos = b -1 + boy * g ;
                    ary[tpos] += ary[lpos];
                }
                if g > 0{
                    let lpos = b + boy * (g-1);
                    ary[tpos] += ary[lpos];
                }
            }
        }
    }

    print!("{}", ary[size-2] + ary[size-boy-1]);


}