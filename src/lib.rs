mod zaphkiel;

use cargo_leet::*;
use std::cell::RefCell;
use std::rc::Rc;

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
    use std::sync::RwLock;
    fn sort_word(s: &String) -> String {
        let mut vec = s.chars().collect::<Vec<_>>();
        vec.sort_unstable();
        vec.iter().collect()
    }

    let mut map: HashMap<String, RwLock<Vec<String>>> = HashMap::new();

    for word in strs {
        let sorted_word = sort_word(&word);
        if let Some(existing_word) = map.get(&sorted_word) {
            existing_word.write().unwrap().push(word);
        } else {
            let mut new_vec = vec![];
            new_vec.push(word);
            map.insert(sorted_word, RwLock::new(new_vec));
        }
    }

    todo!()
}

pub fn hr_simple_array_sum(ar: &[i32]) -> i32 {
    ar.iter().sum()
}

pub fn hr_compare_the_triplets(inp: (&[i32], &[i32])) -> Vec<i32> {
    let (a, b) = inp;

    let mut alice = 0;
    let mut bob = 0;

    a.iter().zip(b).for_each(|(a, b)| {
        if a > b {
            alice += 1;
        } else if a < b {
            bob += 1;
        }
    });

    vec![alice, bob]
}

pub fn a_very_big_sum(ar: &[i64]) -> i64 {
    ar.iter().sum()
}

pub fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let length = arr.len();

    let primary_diagonal: i32 = (0..length).map(|i| arr[i][i]).sum();
    let secondary_diagonal: i32 = (0..length).map(|i| arr[(length - 1) - i][i]).sum();

    (primary_diagonal - secondary_diagonal).abs()
}

pub fn plus_minus(arr: &[i32]) -> bool {
    let mut positive = 0;
    let mut negative = 0;
    let mut zeros = 0;

    for i in arr {
        if i.is_positive() {
            positive += 1;
        } else if i.is_negative() {
            negative += 1;
        } else {
            zeros += 1;
        }
    }

    assert_eq!(positive + negative + zeros, arr.len());

    let total = positive + negative + zeros;

    let positive = positive as f64 / total as f64;
    let negative = negative as f64 / total as f64;
    let zeros = zeros as f64 / total as f64;

    println!("{:.6}\n{:.6}\n{:.6}", positive, negative, zeros);

    true
}

pub fn wheresmyinternet(input: String) -> String {
    let mut lines = input.split('\n');
    let (first, rest) = (lines.next().unwrap().to_owned(), lines);

    let (no_houses, _) = first.split_once(' ').unwrap();
    let no_houses = no_houses.parse::<usize>().unwrap();

    use std::collections::{HashMap, HashSet};

    let rest = rest
        .map(|s| s.split_once(' ').unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()));

    let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();

    fn connect(map: &mut HashMap<usize, HashSet<usize>>, a: usize, b: usize) {
        map.entry(a)
            .and_modify(|v| {
                v.insert(b);
            })
            .or_insert({
                let mut set = HashSet::new();
                set.insert(b);
                set
            });
        map.entry(b)
            .and_modify(|v| {
                v.insert(a);
            })
            .or_insert({
                let mut set = HashSet::new();
                set.insert(a);
                set
            });
    }

    connect(&mut map, 1, 1);
    for (a, b) in rest {
        connect(&mut map, a, b);
    }

    let mut flags = vec![false; no_houses];

    fn descent(
        map: &HashMap<usize, HashSet<usize>>,
        ele: usize,
        max: usize,
        flags: &mut Vec<bool>,
    ) {
        flags[ele - 1] = true;
        let binding = HashSet::new();
        let cons = map.get(&ele).unwrap_or(&binding); // Use unwrap_or to handle None

        for &neighbor in cons {
            if flags[neighbor - 1] {
                dbg!(neighbor, cons);
            }
            if !flags[neighbor - 1] {
                descent(map, neighbor, max - 1, flags);
            }
        }
    }

    descent(&map, 1, no_houses, &mut flags);

    let unvisited = flags
        .iter()
        .enumerate()
        .filter(|(_idx, flag)| !**flag)
        .map(|(idx, _flag)| idx + 1)
        .collect::<Vec<_>>();

    if unvisited.is_empty() {
        "Connected".to_string()
    } else {
        unvisited
            .iter()
            .map(|x| format!("{}\n", x))
            .collect::<String>()
            .trim()
            .to_string()
    }
}

// pub fn amount_of_time_2385(args: (Option<Rc<RefCell<TreeNode>>>, i32)) -> i32 {
//     let (root, start) = args;
//     // this works only when the starting value is on the shorter branch
//     pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
//         let left_sub_tree = root.clone().unwrap().borrow().left.clone();
//         let right_sub_tree = root.clone().unwrap().borrow().right.clone();

//         pub fn find_depth(
//             node: Option<Rc<RefCell<TreeNode>>>,
//             val: i32,
//             curr: usize,
//         ) -> Option<usize> {
//             if let Some(node) = node {
//                 if node.borrow().val == val {
//                     // found the starting node
//                     Some(curr)
//                 } else {
//                     let left_node = node.borrow().left.clone();
//                     let right_node = node.borrow().right.clone();

//                     if let Some(found) = find_depth(left_node, val, curr + 1) {
//                         // found it in the left sub-tree
//                         Some(found)
//                     } else if let Some(found) = find_depth(right_node, val, curr + 1) {
//                         // found it in the right sub-tree
//                         Some(found)
//                     } else {
//                         // not present in the current sub-tree
//                         None
//                     }
//                 }
//             } else {
//                 // went past the leaf node
//                 None
//             }
//         }

//         let find_in_left = find_depth(left_sub_tree.clone(), start, 0);
//         let find_in_right = find_depth(right_sub_tree.clone(), start, 0);

//         pub fn find_deepest(
//             node: Option<Rc<RefCell<TreeNode>>>,
//             highest: usize,
//             curr: usize,
//         ) -> usize {
//             if let Some(node) = node {
//                 let left_node = node.borrow().left.clone();
//                 let right_node = node.borrow().right.clone();

//                 let left_depth = find_deepest(left_node, highest, curr + 1);
//                 let right_depth = find_deepest(right_node, highest, curr + 1);

//                 let deeper = if left_depth > right_depth {
//                     left_depth
//                 } else {
//                     right_depth
//                 };

//                 if highest < deeper {
//                     deeper
//                 } else {
//                     highest
//                 }
//             } else {
//                 // went past the leaf node
//                 if highest < curr {
//                     // found a new record depth
//                     curr
//                 } else {
//                     // found a smaller depth
//                     highest
//                 }
//             }
//         }

//         let deepest_left = find_deepest(left_sub_tree, 0, 0);
//         let deepest_right = find_deepest(right_sub_tree, 0, 0);

//         let deepest = if deepest_left > deepest_right {
//             deepest_left
//         } else {
//             deepest_right
//         };

//         match (find_in_left, find_in_right, deepest_left > deepest_right) {
//             (Some(depth), None, true) => {
//                 // found in left + left is deeper
//                 let from_node_to_deepest = TryInto::<i32>::try_into(deepest - depth).unwrap() - 1;

//                 let from_node_to_

//             }
//             (None, Some(depth), false) => {
//                 // found in right + right is deeper
//                 let node_to_deepest = TryInto::<i32>::try_into(deepest - depth).unwrap() - 1;

//                 depth.try_into().unwrap()
//             }
//             (Some(depth), None, false) => {
//                 // found in left + right is deeper

//                 TryInto::<i32>::try_into(deepest + depth).unwrap() + 1
//             }
//             (None, Some(depth), true) => {
//                 // found in right + left is deeper
//                 TryInto::<i32>::try_into(deepest + depth).unwrap() + 1
//             }
//             (None, None, _) => {
//                 // root node was the starting point
//                 deepest.try_into().unwrap()
//             }
//             huh => {
//                 panic!("this should not be possible: {huh:?}")
//             }
//         }
//     }
//     amount_of_time(root, start)
// }

pub fn max_ancestor_diff_1026(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    pub fn recurse(node: Option<Rc<RefCell<TreeNode>>>, min_so_far: i32, max_so_far: i32) -> i32 {
        if let Some(node) = node {
            let val = node.borrow().val;
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();

            let min = val.min(min_so_far);
            let max = val.max(max_so_far);

            let left_result = recurse(left, min, max);
            let right_result = recurse(right, min, max);

            left_result.max(right_result)
        } else {
            max_so_far - min_so_far
        }
    }

    recurse(
        root.clone(),
        root.clone().unwrap().borrow().val,
        root.clone().unwrap().borrow().val,
    )
}

pub fn halves_are_alike_1704(s: String) -> bool {
    let (left, right) = s.split_at(s.len() / 2);

    let (left, right) = (left.to_lowercase(), right.to_lowercase());

    left.chars()
        .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
        == right
            .chars()
            .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
            .count()
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

    generate_tests! {MAP;
        test_217_contains_duplicate_1, contains_duplicate_217, vec![1,2,3,1], true;
        test_217_contains_duplicate_2, contains_duplicate_217, vec![1,2,3,4], false;
        test_217_contains_duplicate_3, contains_duplicate_217, vec![1,1,1,3,3,4,3,2,4,2], true;

        test_242_valid_anagram_1, valid_anagram_242, ("anagram".into(), "nagaram".into()), true;
        test_242_valid_anagram_2, valid_anagram_242, ("rat".into(), "car".into()), false;

        test_1_two_sum_1, two_sum_1, (vec![2,7,11,15], 9), vec![0, 1];
        test_1_two_sum_2, two_sum_1, (vec![3,2,4], 6), vec![1,2];
        test_1_two_sum_3, two_sum_1, (vec![3,3], 6), vec![0,1];

        /* test_49_group_anagrams_1, */

        test_hr_simple_array_sum_1, hr_simple_array_sum, &[1, 2, 3, 4, 10, 11], 31;
        test_hr_compare_the_triplets, hr_compare_the_triplets, (&[5, 6, 7], &[3, 6, 10]), vec![1, 1];
        test_hr_a_very_big_sum, a_very_big_sum, &[1000000001, 1000000002, 1000000003, 1000000004, 1000000005], 5000000015;
        test_hr_diagonal_difference, diagonal_difference, &[vec![11, 2, 4], vec![4, 5, 6], vec![10, 8, -12]], 15;
        test_hr_plus_minus, plus_minus, &[-4, 3, -9, 0, 4, 1], true;

        test_kattis_wheresmyinternet_1, wheresmyinternet, "2 1\n2 1".to_string(), "Connected".to_string();
        test_kattis_wheresmyinternet_2, wheresmyinternet, "6 4\n1 2\n2 3\n3 4\n5 6".to_string(), "5\n6".to_string();
        test_kattis_wheresmyinternet_3, wheresmyinternet, "4 3\n2 3\n4 2\n3 4".to_string(), "2\n3\n4".to_string();

        // test_2385_amount_of_time_for_binary_tree_to_be_infected_1, amount_of_time_2385, (TreeRoot::from("[1,5,3,null,4,10,6,9,2]").into(), 3), 4;
        // test_2385_amount_of_time_for_binary_tree_to_be_infected_2, amount_of_time_2385, (TreeRoot::from("[1,2,null,3,null,4,null,5]").into(), 2), 3;
        // test_2385_amount_of_time_for_binary_tree_to_be_infected_3, amount_of_time_2385, (TreeRoot::from("[1]").into(), 0), 0;
        // test_2385_amount_of_time_for_binary_tree_to_be_infected_4, amount_of_time_2385, (TreeRoot::from("[1,2,null,3,null,4,null,5]").into(), 4), 3;
        // test_2385_amount_of_time_for_binary_tree_to_be_infected_5, amount_of_time_2385, (TreeRoot::from("[5,2,3,4,null,null,null,1]").into(), 4), 3;
        // test_2385_amount_of_time_for_binary_tree_to_be_infected_6, amount_of_time_2385, (TreeRoot::from("[1,null,2,3,4,null,5]").into(), 4), 3;

        test_1026_maximum_difference_between_node_and_ancestor_1, max_ancestor_diff_1026, TreeRoot::from("[8,3,10,1,6,null,14,null,null,4,7,13]").into(), 7;

        test_1704_determine_if_string_halves_are_alike_1, halves_are_alike_1704, "book".into(), true;

    }

    // #[test]
    // pub fn print_map() {
    //     let no_of_tests = vec![3, 2, 3].iter().sum();
    //     let mut map: HashMap<String, Duration> = MAP.read().unwrap().clone();

    //     loop {
    //         if map.len() != no_of_tests {
    //             dbg!(map.len() != no_of_tests, map.len(), no_of_tests);
    //             std::thread::sleep(Duration::from_secs(1));
    //         } else {
    //             break;
    //         }
    //         map = MAP.read().unwrap().clone();
    //     }
    //     let mut map = map.iter().collect::<Vec<(&String, &Duration)>>();
    //     map.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
    //     let map = map
    //         .iter()
    //         .map(|(k, v)| format!("{k} => {v:#?}"))
    //         .collect::<Vec<String>>();
    //     println!("\n\n\n\n\n");
    //     dbg!(map);
    // }
}
