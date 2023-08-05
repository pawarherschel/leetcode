mod zaphkiel;

pub fn contains_duplicate_217(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;

    nums.iter().map(|&v| v).collect::<HashSet<i32>>().len() != nums.len()
}

pub fn valid_anagram_242(input: (String, String)) -> bool {
    let (s, t) = input;

    let mut s = s.chars().collect::<Vec<_>>();
    s.sort_unstable();

    let mut t = t.chars().collect::<Vec<_>>();
    t.sort_unstable();

    s == t
}

pub fn two_sum_1(input: (Vec<i32>, i32)) -> Vec<i32> {
    let (nums, target) = input;

    for (idx, x) in nums.iter().enumerate() {
        for (jdx, y) in nums.iter().enumerate() {
            if idx != jdx && x + y == target {
                return vec![idx as i32, jdx as i32];
            }
        }
    }

    unreachable!();
}

pub fn group_anagrams_49(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    fn sort_word(s: &String) -> String {
        let mut vec = s.chars().collect::<Vec<_>>();
        vec.sort_unstable();
        vec.iter().collect()
    }

    let mut map: HashMap<String, &mut Vec<String>> = HashMap::new();

    let sorted_strs = strs
        .iter()
        .enumerate()
        .map(|(idx, word)| (idx, word.chars().collect::<Vec<_>>()))
        .collect::<Vec<_>>();

    for word in strs {
        let sorted_word = sort_word(&word);
        if let Some(mut existing_word) = map.get(&sorted_word) {
            existing_word.push(word);
        } else {
            let mut new_vec = vec![];
            new_vec.push(word);
            map.insert(sorted_word, &mut new_vec);
        }
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use std::sync::RwLock;
    use std::{collections::HashMap, time::Duration};

    lazy_static! {
        pub static ref MAP: RwLock<HashMap<String, Duration>> = RwLock::new(HashMap::new());
    }

    generate_tests![MAP;
        test_217_contains_duplicate_1, contains_duplicate_217, vec![1,2,3,1], true;
        test_217_contains_duplicate_2, contains_duplicate_217, vec![1,2,3,4], false;
        test_217_contains_duplicate_3, contains_duplicate_217, vec![1,1,1,3,3,4,3,2,4,2], true;

        test_242_valid_anagram_1, valid_anagram_242, ("anagram".into(), "nagaram".into()), true;
        test_242_valid_anagram_2, valid_anagram_242, ("rat".into(), "car".into()), false;

        test_1_two_sum_1, two_sum_1, (vec![2,7,11,15], 9), vec![0, 1];
        test_1_two_sum_2, two_sum_1, (vec![3,2,4], 6), vec![1,2];
        test_1_two_sum_3, two_sum_1, (vec![3,3], 6), vec![0,1];
    ];

    #[test]
    pub fn print_map() {
        let no_of_tests = vec![3, 2, 3].iter().sum();
        let mut map: HashMap<String, Duration> = MAP.read().unwrap().clone();

        loop {
            if map.len() != no_of_tests {
                dbg!(map.len() != no_of_tests, map.len(), no_of_tests);
                std::thread::sleep(Duration::from_secs(1));
            } else {
                break;
            }
            map = MAP.read().unwrap().clone();
        }
        let mut map = map.iter().collect::<Vec<(&String, &Duration)>>();
        map.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
        let map = map
            .iter()
            .map(|(k, v)| format!("{k} => {v:#?}"))
            .collect::<Vec<String>>();
        println!("\n\n\n\n\n");
        dbg!(map);
    }
}
