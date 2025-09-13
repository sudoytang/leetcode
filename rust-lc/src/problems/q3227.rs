#[allow(unused)]
pub struct Solution;
#[allow(unused)]
impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        // case A: no vowels in s: -> xiaoming(bob) wins
        // case B: odd number of vowels in s -> xiaohong(alice) wins because he can just remove everything
        // case C: even number of vowels in s
        // 1. xiaohong remove even numbers of vowels, the remaining contains odd numbers of vowels
        // 2. xiaoming remove odd numbers of vowels, the remaining contains odd numbers of vowels
        // 3. case B

        s.chars().filter(|c| ['a', 'e', 'i', 'o', 'u'].contains(c)).count() != 0
    }
}