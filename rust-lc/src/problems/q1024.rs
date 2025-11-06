#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[allow(unused)]

impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, time: i32) -> i32 {
        let mut clips = clips;
        clips.sort_unstable_by(|a, b| {
            // smallest start + biggest end
            match a[0].cmp(&b[0]) {
                std::cmp::Ordering::Equal => b[1].cmp(&a[1]),
                order => order
            }
        });

        let mut count = 0;
        let mut reached = 0;
        let mut cur_reachable = 0;
        for clip in clips.iter() {
            if reached >= clip[0] {
                cur_reachable = cur_reachable.max(clip[1]);
            } else {
                // can't go anymore
                if clip[0] > cur_reachable {
                    break;
                }
                count += 1;
                reached = cur_reachable;
                cur_reachable = cur_reachable.max(clip[1])
            }
            if reached >= time {
                break;
            }
        }
        if reached >= time {
            count
        } else if cur_reachable >= time {
            count + 1
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_1() {
        let v = [[0,2],[4,6],[8,10],[1,9],[1,5],[5,9]].map(|v| v.into()).into();
        let res = Solution::video_stitching(v, 10);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_2() {
        let v = [[5,7],[1,8],[0,0],[2,3],[4,5],[0,6],[5,10],[7,10]].map(|v| v.into()).into();
        let res = Solution::video_stitching(v, 5);
        assert_eq!(res, 1);
    }
}