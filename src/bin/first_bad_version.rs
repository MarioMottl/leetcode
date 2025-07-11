fn is_bad_version(version: i32) -> bool {
    return false;
}

pub fn first_bad_version(n: i32) -> i32 {
    let (mut left, mut right) = (0, n);

    while left < right {
        let mid = left + (right - left) / 2;
        if is_bad_version(mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn main() {}

#[allow(unused)]
#[cfg(test)]
mod tests {
    use super::*;
    // cant test this as I dont have the API self.isBadVersion(version)
}
