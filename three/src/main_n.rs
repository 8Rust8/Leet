fn main() {
    let s = "pwwkew".to_string();
    println!("s :: {:?}", s);

    let length = length_of_longest_substring(s);
    println!("Max lenght :: {}", length);
}

fn length_of_longest_substring(s: String) -> i32 {
    let s_bytes = s.bytes();
    let mut max_rep_len: usize = 0;
    let mut max_len: usize = 0;

    let mut hash_map: std::collections::HashMap<u8, u8> = std::collections::HashMap::new();

    for (index, val) in s_bytes.enumerate() {
        match hash_map.get(&val) {
            Some(t) => {
                if (index - *t as usize) > max_rep_len && (index - *t as usize) > max_len {
                    max_rep_len = index - *t as usize;
                    if *t == 0 {
                        max_rep_len += 1;
                    }
                    max_len =0 ;
                }
                hash_map.insert(val, index as u8);
            }
            None => {
                hash_map.insert(val, index as u8);
                max_len += 1;
            }
        }
    }

    println!("Max rep len :: {}", max_rep_len);
    println!("Max len :: {}", max_len);
    if max_len > max_rep_len {
        return max_len as i32;
    }
    max_rep_len as i32
}
