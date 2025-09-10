#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;
        use std::collections::HashMap;
        let languages: Vec<HashSet<usize>> = languages
            .into_iter()
            .map(|v| v.into_iter().map(|i| i as usize - 1).collect())
            .collect();

        let n = n as usize;
        let mut unsatisfied_person = HashSet::new();
        for friendship in &friendships {
            if let [a, b] = friendship.as_slice() {
                let person_1 = *a as usize - 1;
                let person_2 = *b as usize - 1;
                if languages[person_1].intersection(&languages[person_2]).next().is_some() {
                    continue;
                } else {
                    unsatisfied_person.insert(person_1);
                    unsatisfied_person.insert(person_2);
                }

            } else {
                unreachable!()
            }
        }
        // some people are not satisfied, to make all of them satisfied, we need to pick up one language
        // and teach them all (but skipping ones that learnt this language)
        // if a language is learned by most people, we can greedily pick it
        let mut lang_learnt_by_unsat: HashMap<usize, usize> = HashMap::new();
        for &person in &unsatisfied_person {
            for &lang in &languages[person] {
                *lang_learnt_by_unsat.entry(lang).or_insert(0) += 1;
            }
        }
        return (unsatisfied_person.len() - lang_learnt_by_unsat.values().cloned().max().unwrap_or(0)) as i32

    }
}