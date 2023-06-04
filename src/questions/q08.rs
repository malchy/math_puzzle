use std::ops;
const N:usize = 12;  // 移動回数

#[derive(Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
impl Point{
    fn origin() -> Point{
        Point{x:0, y:0}
    }
    fn new(x: i32, y:i32) -> Point{
        Point { x: x, y: y }
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, _rhs: Point) -> Point{
        Point::new(self.x + _rhs.x, self.y + _rhs.y)
    }
}

fn move_robot (log : &mut Vec<Point>) -> i32{
    if log.len() == N + 1 {
        // 移動回数Nに到達したので終了
        1
    }else{
        let mut cnt = 0;
        let vector = [
            Point::new(0,1),   // down
            Point::new(0,-1),  // up
            Point::new(1,0),   // right
            Point::new(-1,0)]; // left
        for d in vector {
            let next_pos = Point::new(log.last().unwrap().x + d.x, log.last().unwrap().y + d.y );
            // 探索済みでなければ移動させる
            if !log.contains(&next_pos){
                let mut v = log.to_vec();
                v.push(next_pos);
                cnt += move_robot( &mut v);
            }
        }
        cnt
    }
}

pub fn q08(){
    let mut v = vec![Point::origin()];
    println!("{}", move_robot(&mut v));
}