fn main() {
    let s = "abba".to_string();
    println!("s :: {:?}", s);

    let length = length_of_longest_substring(s);
    println!("Max lenght :: {}", length);
}

fn length_of_longest_substring(s: String) -> i32 {
    let s_bytes = s.bytes();
    let mut new_len: usize = 0;
    let mut max_len: usize = 0;

    let mut hash_map: std::collections::HashMap<u8, u8> = std::collections::HashMap::new();

    for (index, val) in s_bytes.enumerate() {
        match hash_map.get(&val) {
            Some(t) => {
                println!("index:: {:?}", index);
                if (index - *t as usize) > new_len {
                    if (index - *t as usize) > max_len {
                        max_len = index - *t as usize;
                    }
                } else {
                    if new_len > max_len {
                        max_len = new_len;
                    }
                }
                new_len = 1;
            }
            None => {
                new_len += 1;
            }
        }
        println!("Max lenght :: {}", max_len);
        println!("New lenght :: {}", new_len);
        hash_map.insert(val, index as u8);
    }

    if new_len > max_len {
        max_len = new_len;
    }

    max_len as i32
}
