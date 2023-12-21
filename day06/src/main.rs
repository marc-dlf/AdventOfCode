use std::collections::HashMap;

fn count_ways_win(time: usize, distance: usize) -> usize {
    let mut left_bound: usize = distance;
    let mut right_bound: usize = 0;

    //find left border
    for press_time in 0..time + 1 {
        let attempt_distance = (time - press_time) * press_time;
        if attempt_distance > distance {
            left_bound = press_time;
            break;
        }
    }

    //find right border
    for press_time in (0..time + 1).rev() {
        let attempt_distance = (time - press_time) * press_time;
        if attempt_distance > distance {
            right_bound = press_time;
            break;
        }
    }

    match left_bound < right_bound {
        true => 1 + right_bound - left_bound,
        false => 0,
    }
}

//Input
// Time:        53     83     72     88
// Distance:   333   1635   1289   1532

fn main() {
    //Part 1
    // let input: HashMap<usize, usize> =
    //     HashMap::from([(53, 333), (83, 1635), (72, 1289), (88, 1532)]);
    //Part 2
    let input: HashMap<usize, usize> = HashMap::from([(53837288, 333163512891532)]);
    let mut output = 1;
    input
        .iter()
        .for_each(|(t, d)| output *= count_ways_win(*t, *d));
    println!("{:?}", output);
}
