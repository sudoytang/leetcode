#[allow(unused)]
struct Solution;
#[allow(unused)]
impl Solution {

    fn area(p1: (i32, i32), p2: (i32, i32), p3: (i32, i32)) -> f64 {
        let (x1, y1) = p1;
        let (x2, y2) = p2;
        let (x3, y3) = p3;
        let x1 = x1 as i64;
        let x2 = x2 as i64;
        let x3 = x3 as i64;
        let y1 = y1 as i64;
        let y2 = y2 as i64;
        let y3 = y3 as i64;

        let area2x = ((x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)).abs();

        return area2x as f64 / 2.
    }

    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut max = 0f64;
        for i in 0..points.len() {
            for j in i+1..points.len() {
                for k in j+1..points.len() {
                    let p1 = (points[i][0], points[i][1]);
                    let p2 = (points[j][0], points[j][1]);
                    let p3 = (points[k][0], points[k][1]);
                    max = max.max(Self::area(p1, p2, p3));
                }
            }
        }
        max
    }
}