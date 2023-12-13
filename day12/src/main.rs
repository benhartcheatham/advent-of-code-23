use clap::{arg, command, ArgAction};
use std::io;

fn main() -> Result<(), io::Error> {
    let input = include_str!("../../input.txt");
    let example = include_str!("../../example.txt");

    let matches = command!()
        .arg(arg!(example: -e).action(ArgAction::SetTrue))
        .get_matches();

    if matches.get_flag("example") {
        println!("solution (example): {}", solution(example));
    } else {
        println!("solution: {}", solution(input));
    }

    Ok(())
}

/*
 * from here: https://www.reddit.com/r/adventofcode/comments/18ge41g/2023_day_12_solutions/kd18cl9/
 * comments by me :)
 */
fn find_arrangements(springs: &str, groups: &Vec<bool>) -> u64 {
    let mut dp: Vec<Vec<u64>> = Vec::new();
    // dp[i][j] == [character c][possible states of c]
    dp.resize(springs.len() + 1, Vec::new());

    for r in dp.iter_mut() {
        r.resize(groups.len() + 1, 0);
    }

    // "base" case
    dp[springs.len()][groups.len()] = 1;

    // compare spring ('#', '.', or '?') to group[j] ('T', 'F")
    for (i, c) in springs.char_indices().rev() {
        for j in (0..groups.len()).rev() {
            let (mut damaged, mut operational) = (false, false);

            // set what possible state the character could have
            match c {
                '#' => damaged = true,
                '.' => operational = true,
                _ => {
                    damaged = true;
                    operational = true
                }
            }

            let mut sum = 0;

            // if this character could be damaged then we must be in a group
            if damaged && groups[j] {
                sum += dp[i + 1][j + 1];
            // if this character is operational then we must be in a gap of
            // operational springs
            } else if operational && !groups[j] {
                sum += dp[i + 1][j + 1] + dp[i + 1][j];
            }

            // this character is the sum of possibilites based on the
            // 1-2 characters before it
            dp[i][j] = sum;
        }
    }

    dp[0][0]
}

fn make_groups(groups: Vec<usize>) -> Vec<bool> {
    let mut bools = Vec::new();

    bools.push(false);

    for g in groups {
        bools.append(&mut vec![true; g]);

        bools.push(false);
    }

    bools
}

fn solution(input: &str) -> u64 {
    let lines = input.lines();
    let mut sum = 0;

    for line in lines {
        let parts: Vec<_> = line.split(' ').collect();

        if parts.len() != 2 {
            continue;
        }

        let mut springs = (parts[0].to_string() + "?").repeat(5);
        springs.pop();
        springs = ".".to_string() + &springs + ".";
        let groups: Vec<_> = (parts[1].to_string() + ",")
            .repeat(5)
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        sum += find_arrangements(&springs.to_string(), &make_groups(groups));
    }

    sum
}
