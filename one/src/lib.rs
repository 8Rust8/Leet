use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut response: Vec<i32> = Vec::new();
    match nums.is_empty() {
        true => println!("No data found"),
        false => {
            let mut hashmap: HashMap<usize, i32> = HashMap::new();
            for (ind, num) in nums.iter().enumerate() {
                for (key, value) in hashmap.iter() {
                    if value == num {
                        response.push(*key as i32);
                        response.push(ind as i32);
                        break;
                    }
                }
                let compliment = target - num;
                hashmap.insert(ind, compliment);
            }
            println!("final hashmap :: {:?}", hashmap);
        }
    }

    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum() {
        let v = vec![3, 4, 1, 4, 2];
        let result = two_sum(v, 8);
        assert_eq!(result, [0, 1]);
    }
}
