struct Solution;

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        let mut ret = 1;
        points.sort_by(|a,b|{
            a[0].cmp(&b[0])
        });
        for i in 1..points.len() {
            if points[i][0] > points[j][1] {
                ret+=1;
            }else {
                points[i][1] = points[i][1].min(points[i- 1][1])
            }
        }
        ret
    }
}