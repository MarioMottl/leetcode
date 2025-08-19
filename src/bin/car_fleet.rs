pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut cars: Vec<(i32, i32)> = position.into_iter().zip(speed.into_iter()).collect();
    cars.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    let mut fleets = 0;
    let mut last_time = -1.0_f64; // time of the fleet in front

    for (pos, spd) in cars {
        let t = (target - pos) as f64 / spd as f64; // time to reach target
        if t > last_time {
            fleets += 1;
            last_time = t;
        }
    }

    fleets
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let target = 12;
        let position = vec![10, 8, 0, 5, 3];
        let speed = vec![2, 4, 1, 1, 3];
        assert_eq!(car_fleet(target, position, speed), 3);
    }
}
