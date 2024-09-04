use std::collections::HashMap;

fn main() {

}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m: HashMap<i32, i32> = HashMap::new();

    nums.iter().enumerate().find_map(|(i, &v)| {
        match m.get(&(target - v)) {
            Some(&i2) => Some(vec![i as i32, i2]),  // Return the result immediately
            None => {
                m.insert(v, i as i32);
                None  
            }
        }
    }).unwrap_or(vec![])
}


