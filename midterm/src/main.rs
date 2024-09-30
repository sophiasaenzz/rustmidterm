use std::collections::HashMap;


/* Given an array of integers nums and an integer target, 
return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, 
and you may not use the same element twice.

You can return the answer in any order. */

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new(); //make a hash map

    //fill the hash map
    for (idx, &num) in nums.iter().enumerate() {
        map.insert(num, idx);
    }

    //find the complement
    for (idx, &num) in nums.iter().enumerate() {
        let comp = target - num;

        if let Some(&comp_idx) = map.get(&comp) {
            if comp_idx != idx {
                return vec![idx as i32, comp_idx as i32];
            }
        }
    }

    vec![] //return empty if no solution
}

fn hello_world_func() {
    println!("Hello World!");
}

fn main() {
    let nums = vec![2,7,11,15];
    two_sum(nums, 9);

    hello_world_func();
}


