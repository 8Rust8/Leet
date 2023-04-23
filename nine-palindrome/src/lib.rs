#![allow(unused)]

pub mod nine_palindrome {
    pub fn is_palindrome(x: i32) -> bool {
        if (x < 0) {
            return false;
        }
        let div = 10;
        let mut d = x;
        let mut reverse = vec![];
        while d > 9 {
            let r = d % div;
            d = d / div;
            reverse.push(r);
        }
        reverse.push(d);
        println!("reverse : {:?}", reverse);

        // to get te actual number back
        let mut num = 0;
        for (ind, n) in reverse.iter().enumerate() {
            let pow = (reverse.len() - ind) as u32 - 1;
            num += n * 10_i32.pow(pow);
        }
        println!("number  {}", num);

        match x == num {
            true => true,
            false => false,
        }
    }
}

#[cfg(test)]
mod tests;
