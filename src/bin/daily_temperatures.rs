pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let n = temperatures.len();
    let mut answer = vec![0; n];
    let mut stack: Vec<usize> = Vec::new(); // stores indices

    for i in 0..n {
        while let Some(&last) = stack.last() {
            if temperatures[i] > temperatures[last] {
                stack.pop();
                answer[last] = (i - last) as i32;
            } else {
                break;
            }
        }
        stack.push(i);
    }

    answer
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
}
