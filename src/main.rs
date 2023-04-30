mod questions;

fn main() {
    let question_no = 4;

    match question_no{
        1 => questions::q01::q01(),
        2 => questions::q02::q02(),
        3 => {
            questions::q03::q03_1();
            questions::q03::q03_2();
        },
        4 => {
            questions::q04::q04_1();
            questions::q04::q04_2();
        },
        5 => {
            questions::q05::q05_1();
            questions::q05::q05_2();
            questions::q05::q05_3();
        },
        _ => println!("undefined question"),
    }

}
