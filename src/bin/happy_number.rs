pub fn is_happy(n: u32) -> bool {
    fn next_num(mut x: u32) -> u32 {
        let mut total = 0;
        while x > 0 {
            let digit = x % 10;
            total += digit * digit;
            x /= 10;
        }
        total
    }

    let mut slow = n;
    let mut fast = next_num(n);

    while fast != 1 && slow != fast {
        slow = next_num(slow);
        fast = next_num(next_num(fast));
    }

    fast == 1
}

fn main() {}
