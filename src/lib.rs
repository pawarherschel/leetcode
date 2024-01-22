mod zaphkiel;

use cargo_leet::*;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::ops::Not;
use std::rc::Rc;
use std::sync::RwLock;

pub fn contains_duplicate_217(nums: Vec<i32>) -> bool {
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

pub enum FindWinnersLosses {
    ZERO,
    ONE,
    OTHER,
}

pub fn find_winners_2225(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut loses: BTreeMap<i32, i32> = BTreeMap::new();

    for r#match in matches {
        let [winner, loser, ..] = r#match[..] else {
            unreachable!()
        };

        loses
            .entry(loser)
            .and_modify(|it| *it = *it + 1)
            .or_insert(1);

        loses.entry(winner).or_insert(0);
    }

    let mut never_lost = vec![];
    let mut lost_once = vec![];

    for (player, number_of_loses) in loses {
        match number_of_loses {
            0 => never_lost.push(player),
            1 => lost_once.push(player),
            _ => (),
        }
    }

    // let never_lost = loses
    //     .iter()
    //     .filter(|(_, it)| **it == 0)
    //     .map(|(it, _)| *it)
    //     .collect();
    // let lost_once = loses
    //     .iter()
    //     .filter(|(_, it)| **it == 1)
    //     .map(|(it, _)| *it)
    //     .collect();

    vec![never_lost, lost_once]
}

pub fn unique_occurrences_1207(arr: Vec<i32>) -> bool {
    let mut map = BTreeMap::new();
    let mut set = BTreeSet::new();

    for element in arr {
        map.entry(element)
            .and_modify(|it| *it = *it + 1)
            .or_insert(1);
    }

    for (_, number) in map {
        if !set.insert(number) {
            return false;
        }
    }

    return true;
}

pub fn climb_stairs_70(n: i32) -> i32 {
    match n {
        0 => return 1,
        1 => return 1,
        _ => {}
    }
    let mut n0 = 1;
    let mut n1 = 1;
    let mut n2 = n0 + n1;

    for _ in 2..n {
        n0 = n1;
        n1 = n2;
        n2 = n1 + n0;
    }

    return n2;
}
pub fn min_falling_path_sum_931(matrix: Vec<Vec<i32>>) -> i32 {
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy)]
    struct Pos {
        x: i32,
        y: i32,
    }

    fn get_lmr(pos: Pos, matrix_side: i32) -> (Option<Pos>, Option<Pos>, Option<Pos>) {
        let Pos { x, y } = pos;

        let new_y = y + 1;

        if new_y >= matrix_side {
            return (None, None, None);
        }

        let left = if x - 1 < 0 {
            None
        } else {
            Some(Pos { x: x - 1, y: new_y })
        };

        let middle = Some(Pos { x, y: new_y });

        let right = if x + 1 >= matrix_side {
            None
        } else {
            Some(Pos { x: x + 1, y: new_y })
        };

        (left, middle, right)
    }

    fn recurse(
        matrix: &Vec<Vec<i32>>,
        current_pos: Option<Pos>,
        sum: i32,
        cache: &mut BTreeMap<Pos, i32>,
    ) -> i32 {
        if current_pos.is_none() {
            return sum;
        }
        let current_pos = current_pos.unwrap();

        if let Some(&answer) = cache.get(&current_pos) {
            return answer;
        }

        let Pos { x, y } = current_pos.clone();
        let x = x as usize;
        let y = y as usize;

        let (left, middle, right) = get_lmr(current_pos, matrix.len() as i32);

        let left_val = recurse(matrix, left, sum, cache);
        let middle_val = recurse(matrix, middle, sum, cache);
        let right_val = recurse(matrix, right, sum, cache);

        let minimum = match (left, middle, right) {
            (None, None, None) => {
                let answer = sum + matrix[y][x];
                cache.insert(current_pos, answer);
                return answer;
            }

            (Some(_), None, None) => left_val,
            (None, Some(_), None) => middle_val,
            (None, None, Some(_)) => right_val,

            (Some(_), None, Some(_)) => left_val.min(right_val),
            (Some(_), Some(_), None) => left_val.min(middle_val),
            (None, Some(_), Some(_)) => middle_val.min(right_val),

            (Some(_), Some(_), Some(_)) => left_val.min(middle_val).min(right_val),
        };

        let answer = minimum + sum + matrix[y][x];
        cache.insert(current_pos, answer);
        answer
    }

    let mut cache = std::collections::BTreeMap::new();

    (0..matrix.len())
        // (0..1)
        .map(|x| recurse(&matrix, Some(Pos { x: x as i32, y: 0 }), 0, &mut cache))
        .min()
        .unwrap()
}

pub fn find_error_nums_645(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() == 2 {
        return match nums.as_slice() {
            &[1, 1] => vec![1, 2],
            &[2, 2] => vec![2, 1],
            _ => unreachable!(),
        };
    }

    let mut set = BTreeSet::new();
    let mut dup = None;
    let mut mis = None;

    nums.into_iter().for_each(|n| {
        set.insert(n).not().then(|| dup = Some(n));
    });

    (1..set.last().unwrap() + 2)
        .any(|check_me| set.insert(check_me).then(|| mis = Some(check_me)).is_some());

    vec![dup.unwrap(), mis.unwrap()]
}

#[cfg(test)]
mod tests {
    use super::*;

    generate_tests! {
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

     test_2225_find_players_with_zero_or_one_losses_1, find_winners_2225, vec![vec![1,3],vec![2,3],vec![3,6],vec![5,6],vec![5,7],vec![4,5],vec![4,8],vec![4,9],vec![10,4],vec![10,9]], vec![vec![1,2,10],vec![4,5,7,8]];

     test_1207_unique_number_of_occurrences_1, unique_occurrences_1207,  vec![1,2,2,1,1,3], true;

     test_70_climbing_stairs_1, climb_stairs_70, 2, 2;
     test_70_climbing_stairs_2, climb_stairs_70, 3, 3;
     test_70_climbing_stairs_3, climb_stairs_70, 44, 1134903170;

     test_931_minimum_dalling_path_sum_1, min_falling_path_sum_931, vec![vec![2,1,3],vec![6,5,4],vec![7,8,9]], 13;
     test_931_minimum_dalling_path_sum_2, min_falling_path_sum_931, vec![vec![-19,57],vec![-40,-5]], -59;
     test_931_minimum_dalling_path_sum_3, min_falling_path_sum_931,
        vec![vec![-19,-1,-96,48,-94,36,16,55,-42,37,-59,6,-32,96,95,-58,13,-34,94,85],vec![17,44,36,-29,84,80,-34,50,-99,64,13,91,-27,25,-36,57,20,98,-100,-72],vec![-92,-75,86,90,-4,90,64,56,50,-63,10,-15,90,-66,-66,32,-69,-78,1,60],vec![21,51,-47,-43,-14,99,44,90,8,11,99,-62,57,59,69,50,-69,32,85,13],vec![-28,90,12,-18,23,61,-55,-97,6,89,36,26,26,-1,46,-50,79,-45,89,86],vec![-85,-10,49,-10,2,62,41,92,-67,85,86,27,89,-50,77,55,22,-82,-94,-98],vec![-50,53,-23,55,25,-22,76,-93,-7,66,-75,42,-35,-96,-5,4,-92,13,-31,-100],vec![-62,-78,8,-92,86,69,90,-37,81,97,53,-45,34,19,-19,-39,-88,-75,-74,-4],vec![29,53,-91,65,-92,11,49,26,90,-31,17,-84,12,63,-60,-48,40,-49,-48,88],vec![100,-69,80,11,-93,17,28,-94,52,64,-86,30,-9,-53,-8,-68,-33,31,-5,11],vec![9,64,-31,63,-84,-15,-30,-10,67,2,98,73,-77,-37,-96,47,-97,78,-62,-17],vec![-88,-38,-22,-90,54,42,-29,67,-85,-90,-29,81,52,35,13,61,-18,-94,61,-62],vec![-23,-29,-76,-30,-65,23,31,-98,-9,11,75,-1,-84,-90,73,58,72,-48,30,-81],vec![66,-33,91,-6,-94,82,25,-43,-93,-25,-69,10,-71,-65,85,28,-52,76,25,90],vec![-3,78,36,-92,-52,-44,-66,-53,-55,76,-7,76,-73,13,-98,86,-99,-22,61,100],vec![-97,65,2,-93,56,-78,22,56,35,-24,-95,-13,83,-34,-51,-73,2,7,-86,-19],vec![32,94,-14,-13,-6,-55,-21,29,-21,16,67,100,77,-26,-96,22,-5,-53,-92,-36],vec![60,93,-79,76,-91,43,-95,-16,74,-21,85,43,21,-68,-32,-18,18,100,-43,1],vec![87,-31,26,53,26,51,-61,92,-65,17,-41,27,-42,-14,37,-46,46,-31,-74,23],vec![-67,-14,-20,-85,42,36,56,9,11,-66,-59,-55,5,64,-29,77,47,44,-33,-77]],
        -1428;

    test_645_set_mismatch_1, find_error_nums_645, vec![1,2,2,4], vec![2,3];
    test_645_set_mismatch_2, find_error_nums_645, vec![3,2,2], vec![2,1];
    test_645_set_mismatch_3, find_error_nums_645, vec![1,1], vec![1,2];
    test_645_set_mismatch_4, find_error_nums_645, vec![2,2], vec![2,1];
    test_645_set_mismatch_5, find_error_nums_645, vec![1,3,3], vec![3,2];
    test_645_set_mismatch_6, find_error_nums_645, vec![3,2,3,4,6,5], vec![3,1];
    test_645_set_mismatch_7, find_error_nums_645, vec![2, 3, 2], vec![2,1];
    }
}
