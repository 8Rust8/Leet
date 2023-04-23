#[allow(unused)]
fn main() {
    let s = "abcab~~~cb-+b".to_string();
    let mut s_chars = s.char_indices();
    let mut max_indices = (0, 1);
    let mut max_len = max_indices.1 - max_indices.0;

    'outer: loop {
        match s_chars.next() {
            Some(t) => {
                println!("from in1_chars {:?}", t);
                let mut in2_chars = s_chars.clone();
                'inner: loop {
                    match in2_chars.next() {
                        Some(u) => {
                           // println!("from in2_chars {:?}", u);
                            if t.1 == u.1 {
                                //println!("matched at :: {:?}", u.0);
                                //println!("slice :: {:?}", &s[t.0..u.0]);
                                //println!("indices :: {:?}", (t.0, u.0));
                                if (u.0 - t.0) > max_len {
                                    max_len = u.0 - t.0;
                                    max_indices = (t.0, u.0);
                                }
                                //matched = true;
                                break 'inner;
                            }
                        }
                        None => {
                            break 'inner;
                        }
                    }
                }
            }
            None => break 'outer,
        }
    }

    println!(
        "Max lenght :: {} , slice :: {:?}",
        max_len,
        &s[max_indices.0..max_indices.1]
    );
}
