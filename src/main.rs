use std::fs::File;
use std::io::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let count = find_xmas(input);
        assert_eq!(count, 18);
    }
}

fn main() {
    let mut file = File::open("input.txt").expect("can't open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("cannot read the file");

    let count = find_xmas(contents.as_str());
    print!("found {} xmas", count)
}

fn find_xmas(input: &str) -> i32 {
    let mut candidates: Vec<Candidate> = Vec::new();

    let mut letters: Vec<Vec<Letter>> = Vec::new();
    // read text into memory
    for (y, line) in input.lines().enumerate() {
        let mut line_letters: Vec<Letter> = Vec::new();

        for (x, char) in line.chars().enumerate() {
            if char == 'x' {
                candidates.push(Candidate {
                    pos_x: x as i32,
                    pos_y: y as i32,
                    next: 'x',
                    counter: 0,
                })
            } else {
                line_letters.push(Letter {
                    pos_x: x as i32,
                    pos_y: y as i32,
                    value: char,
                })
            }

            if x == line.len() - 1 {
                letters.push(line_letters)
            }
        }
    }

    let text: [char; 3] = ['M', 'A', 'S'];
    let mut count: i32 = 0;

    for mut c in candidates {
        for y in -1..=1 {
            if c.pos_y <= 3 && y <= 0 {
                // skip top
                continue;
            }
            if c.pos_y <= letters.len() as i32 - 1 && y >= 0 {
                // skip bottom
                continue;
            }
            for x in -1..=1 {
                if c.pos_x <= 3 && x < 0 {
                    // skip left
                    continue;
                }
                if c.pos_x <= letters[0].len() as i32 {
                    // skip right
                    continue;
                }

                if letters[(c.pos_y + y) as usize][(c.pos_x + x) as usize].value == c.next {
                    // need to check the next couple in this direction and inrement counter if
                    // satisfied
                    if letters[(c.pos_y + y * 2) as usize][(c.pos_x + x * 2) as usize].value == 'A'
                    {
                        if letters[(c.pos_y + (y * 3)) as usize][(c.pos_x + (x * 3)) as usize].value
                            == 'S'
                        {
                            c.counter += 1
                        }
                    }
                }
            }
        }

        count += c.counter

        // count += check_neighbors(c, letters)
    }

    count
    // traverse text forward, backward, up, down, diagonal?
    // traverse text once, tracking all permutations that could become XMAS?
    //
    // traverse text, looking for x
    // for each X, count how many of the surrounding up to 8 directions satisfy MAS as well
}

struct Candidate {
    pos_x: i32,
    pos_y: i32,
    next: char,
    counter: i32,
}

struct Letter {
    pos_x: i32,
    pos_y: i32,
    value: char,
}

// fn check_neighbors(c: Candidate, letters: Vec<Candidate>) i32 {
// }
