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
    println!("found {} xmas", count)
}

fn find_xmas(input: &str) -> i32 {
    let mut candidates: Vec<Candidate> = Vec::new();

    let mut letters: Vec<Vec<Letter>> = Vec::new();
    // read text into memory
    for (y, line) in input.lines().enumerate() {
        let mut line_letters: Vec<Letter> = Vec::new();
        for (x, char) in line.chars().enumerate() {
            if char == 'X' {
                candidates.push(Candidate {
                    pos_x: x as i32,
                    pos_y: y as i32,
                    next: 'M',
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
                letters.push(line_letters);
                break;
            }
        }
    }

    let text: [char; 3] = ['M', 'A', 'S'];
    let mut count: i32 = 0;
    let mut x_count: i32 = 0;

    // println!("count of candidates: {}", candidates.len());

    for mut c in candidates {
        x_count += 1;

        for y_offset in -1..=1 as i32 {
            for x_offset in -1..=1 as i32 {
                for (step, next_letter) in text.into_iter().enumerate() {
                    let x_loc = (c.pos_y + (x_offset * step as i32)) as usize;
                    let row = letters.get(x_loc);
                    if row.is_none() {
                        continue;
                    }
                    let y_loc = (c.pos_y + (y_offset * step as i32)) as usize;
                    let col = row.unwrap().get(y_loc);
                    if col.is_none() {
                        continue;
                    }
                    let letter = col.unwrap();

                    if letter.value != next_letter {
                        continue;
                    }

                    println!(
                        "for candidate at {},{}, letter at {},{} is '{}', looking for '{}'",
                        c.pos_x, c.pos_y, x_loc, y_loc, letter.value, next_letter
                    );

                    if letter.value == 'S' {
                        c.counter += 1;
                    }
                    // need to check the next couple in this direction and inrement counter if
                    // satisfied
                    //
                    // let row_a = letters.get((c.pos_y + (y_offset * 2)) as usize);
                    // if row_a.is_none() {
                    //     continue;
                    // }
                    // let col_a = row.unwrap().get((c.pos_x + (x_offset * 2)) as usize);
                    // if col_a.is_none() {
                    //     continue;
                    // }

                    // if letters[(c.pos_y + (y_offset * 2)) as usize]
                    //     [(c.pos_x + (x_offset * 2)) as usize]
                    //     .value
                    //     == 'A'
                    // {
                    //     if letters[(c.pos_y + (y_offset * 3)) as usize]
                    //         [(c.pos_x + (x_offset * 3)) as usize]
                    //         .value
                    //         == 'S'
                    //     {
                    //         c.counter += 1
                    //     }
                    // }
                }
            }
        }

        count += c.counter

        // count += check_neighbors(c, letters)
    }

    println!("Number of X's found: {}", x_count);

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
