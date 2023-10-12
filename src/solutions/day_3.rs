fn priority(c: char) -> u32 {
    let c = c as u32;

    if c >= 97 {
        c - 96
    } else {
        c - 38
    }
}

pub fn solve_1(input: &[&str]) -> u32 {
    let mut priority_sum = 0;

    for line in input {
        let (s, e) = line.split_at(line.len() / 2);

        // this is an n^2 algorithm
        // a quicker option is to sort the string
        // and then find the equal pair
        for c in line.chars() {
            if e.contains(c) {
                priority_sum += priority(c);
                break;
            }
        }
    }

    priority_sum
}

pub fn solve_2(input: &[&str]) -> u32 {
    let mut priority_sum = 0;

    for i in 0..input.len() / 3 {
        let f = input[i * 3];
        let s = input[i * 3 + 1];
        let t = input[i * 3 + 2];

        for c in f.chars() {
            if s.contains(c) && t.contains(c) {
                priority_sum += priority(c);
                break;
            }
        }
    }

    priority_sum
}