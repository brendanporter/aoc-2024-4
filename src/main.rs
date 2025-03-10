use std::fs::File;
use std::io::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_xmas() {
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

    #[test]
    fn test_find_mas() {
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

        let count = find_mas(input);
        assert_eq!(count, 9);
    }
}

fn main() {
    let mut file = File::open("input.txt").expect("can't open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("cannot read the file");

    let count = find_xmas(contents.as_str());
    println!("found {} xmas", count);

    let count = find_mas(contents.as_str());
    println!("found {} mas", count);
}

fn find_mas(input: &str) -> i32 {
    let mut candidates: Vec<Candidate> = Vec::new();

    let mut letters: Vec<Vec<Letter>> = Vec::new();
    // read text into memory
    for (y, line) in input.lines().enumerate() {
        let mut line_letters: Vec<Letter> = Vec::new();
        for (x, char) in line.chars().enumerate() {
            if char == 'A' {
                candidates.push(Candidate {
                    pos_x: x as i32,
                    pos_y: y as i32,
                    counter: 0,
                })
            }
            line_letters.push(Letter { value: char });

            if x == line.len() - 1 {
                letters.push(line_letters);
                break;
            }
        }
    }

    // let text: [char; 2] = ['M', 'S'];
    let mut count: i32 = 0;
    // let mut x_count: i32 = 0;

    // println!("count of candidates: {}", candidates.len());

    for c in candidates {
        // x_count += 1;

        // println!("checking candidate {} at {},{}", i, c.pos_x, c.pos_y);
        let x_offset: i32 = -1;
        let y_offset: i32 = -1;

        // must be surrounded on all corners by opposing 'M' and 'S'
        let row = letters.get((c.pos_y + y_offset) as usize);
        if row.is_none() {
            continue;
        }
        let col = row.unwrap().get((c.pos_x + x_offset) as usize);
        if col.is_none() {
            continue;
        }

        let letter1 = col.unwrap().value;
        if letter1 != 'M' && letter1 != 'S' {
            continue;
        }

        let oppo_row = letters.get((c.pos_y + (y_offset * -1)) as usize);
        if oppo_row.is_none() {
            continue;
        }
        let oppo_col = oppo_row.unwrap().get((c.pos_x + (x_offset * -1)) as usize);
        if oppo_col.is_none() {
            continue;
        }

        let letter2 = col.unwrap().value;
        if letter2 != 'M' && letter2 != 'S' {
            continue;
        }

        if letter1 == letter2 {
            continue;
        }

        let x_offset: i32 = -1;
        let y_offset: i32 = 1;

        // must be surrounded on all corners by opposing 'M' and 'S'
        let row = letters.get((c.pos_y + y_offset) as usize);
        if row.is_none() {
            continue;
        }
        let col = row.unwrap().get((c.pos_x + x_offset) as usize);
        if col.is_none() {
            continue;
        }

        let letter1 = col.unwrap().value;
        if letter1 != 'M' && letter1 != 'S' {
            continue;
        }

        let oppo_row = letters.get((c.pos_y + (y_offset * -1)) as usize);
        if oppo_row.is_none() {
            continue;
        }
        let oppo_col = oppo_row.unwrap().get((c.pos_x + (x_offset * -1)) as usize);
        if oppo_col.is_none() {
            continue;
        }

        let letter2 = col.unwrap().value;
        if letter2 != 'M' && letter2 != 'S' {
            continue;
        }

        if letter1 == letter2 {
            continue;
        }

        count += 1
    }

    count
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
                    counter: 0,
                })
            }
            line_letters.push(Letter { value: char });

            if x == line.len() - 1 {
                letters.push(line_letters);
                break;
            }
        }
    }

    let text: [char; 3] = ['M', 'A', 'S'];
    let mut count: i32 = 0;
    // let mut x_count: i32 = 0;

    // println!("count of candidates: {}", candidates.len());

    for mut c in candidates {
        // x_count += 1;

        // println!("checking candidate {} at {},{}", i, c.pos_x, c.pos_y);
        for y_offset in -1..=1 as i32 {
            for x_offset in -1..=1 as i32 {
                if y_offset == 0 && x_offset == 0 {
                    continue;
                }

                for (step, next_letter) in text.into_iter().enumerate() {
                    let y_loc = (c.pos_y + (y_offset * (step + 1) as i32)) as usize;
                    let row = letters.get(y_loc);
                    if row.is_none() {
                        continue;
                    }
                    let x_loc = (c.pos_x + (x_offset * (step + 1) as i32)) as usize;
                    let col = row.unwrap().get(x_loc);
                    if col.is_none() {
                        continue;
                    }
                    let letter = col.unwrap();

                    // println!("checking for {} at {},{}", next_letter, x_loc, y_loc);

                    if letter.value != next_letter {
                        break;
                    }

                    println!(
                        "for candidate at {},{}, letter at {},{} is '{}', looking for '{}'",
                        c.pos_x, c.pos_y, x_loc, y_loc, letter.value, next_letter
                    );

                    if letter.value == 'S' {
                        c.counter += 1;
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
    counter: i32,
}

struct Letter {
    value: char,
}

// fn check_neighbors(c: Candidate, letters: Vec<Candidate>) i32 {
// }
