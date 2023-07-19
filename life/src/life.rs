use nanorand::{pcg64::Pcg64, Rng};

/// Return `true` iff all cells are off.
pub fn done(fb: &[[u8; 5]; 5]) -> bool {
    fb.iter().all(|&row| row.iter().all(|&v| v == 0))
}

/// Run one turn of Life.
pub fn life(fb: &mut [[u8; 5]; 5]) {
    let prev = *fb;
    for row in 0..5 {
        for col in 0..5 {
            let prev_row = (row + 4) % 5;
            let next_row = (row + 1) % 5;
            let prev_col = (col + 4) % 5;
            let next_col = (col + 1) % 5;
            let coords = [
                (prev_row, prev_col),
                (prev_row, col),
                (prev_row, next_col),
                (row, prev_col),
                (row, next_col),
                (next_row, prev_col),
                (next_row, col),
                (next_row, next_col),
            ];
            let neighbors = coords
                .into_iter()
                .map(|(r, c)| prev[r][c])
                .sum();
            #[allow(clippy::manual_range_contains)]
            match (prev[row][col], neighbors) {
                (1, n) if n < 2 || n > 3 => fb[row][col] = 0,
                (0, 3) => fb[row][col] = 1,
                (_, _) => (),
            }
        }
    }
}

// Fills array with random 1's and 0's 
pub fn random(fb: &mut [[u8; 5]; 5], fr: u128) {
    let mut rng: Pcg64 = nanorand::Pcg64::new_seed(fr * 13);
    let mut b: bool;

    for row in 0..5 {
        for col in 0..5 {
            b = rng.generate();
            if b {
                fb[row][col] = 1
            } else {
                fb[row][col] = 0
            }
        }
    }
}

// Flips all values in the array
pub fn complement(fb: &mut [[u8; 5]; 5]) {
    for row in 0..5 {
        for col in 0..5 {
            if fb[row][col] == 1 {
                fb[row][col] = 0
            } else {
                fb[row][col] = 1
            }
        }
    }
}
