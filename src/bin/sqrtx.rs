pub fn my_sqrt(x: i32) -> i32 {
    if x < 0 {
        panic!("Negative input not allowed");
    }
    if x == 0 || x == 1 {
        return x;
    }

    let mut low = 1;
    let mut high = x / 2;
    let mut result = 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        let square = mid.checked_mul(mid);

        match square {
            Some(sq) if sq == x => return mid,
            Some(sq) if sq < x => {
                result = mid;
                low = mid + 1;
            }
            _ => {
                high = mid - 1;
            }
        }
    }

    result
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
}
