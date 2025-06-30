const MINE: char = '*';

fn main() {
    annotate(&[
         "  *  ",
        "  *  ",
        "*****",
        "  *  ",
        "  *  ",
    ]);
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let l = minefield.len();

    if l == 0 {
        return ([]).to_vec();
    }

    // warning: manual saturating add detected
//   --> src/lib.rs:84:9
//    |
// 84 | /         if *v != u8::MAX {
// 85 | |             *v += 1;
// 86 | |         }
//    | |_________^ help: use instead: `*v = *v.saturating_add(1);`
//    |
//    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#implicit_saturating_add
//    = note: `#[warn(clippy::implicit_saturating_add)]` on by default

    let mut annotated: Vec<String> = vec!["".to_string(); l];
    let w = minefield[0].len();
    let mut matrix: Vec<u8> = vec![0; l * w];

    for (row_idx, row) in minefield.iter().enumerate() {
        for (cell_idx, cell) in row.as_bytes().iter().enumerate() {
            if *cell as char == MINE {
                matrix[row_idx * w + cell_idx] = u8::MAX;

                if cell_idx > 0 {
                    let left = row_idx * w + cell_idx - 1;
                    increment_cell(matrix.get_mut(left));

                    if row_idx > 0 {
                        let left_top = (row_idx - 1) * w + cell_idx - 1;
                        increment_cell(matrix.get_mut(left_top));
                    }

                    if row_idx < l {
                        let left_bottom = (row_idx + 1) * w + cell_idx - 1;
                        increment_cell(matrix.get_mut(left_bottom));
                    }
                }

                if cell_idx < w - 1 {
                    let right = row_idx * w + cell_idx + 1;
                    increment_cell(matrix.get_mut(right));

                    if row_idx > 0 {
                        let right_top = (row_idx - 1) * w + cell_idx + 1;
                        increment_cell(matrix.get_mut(right_top));
                    }

                    if row_idx < l {
                        let right_bottom = (row_idx + 1) * w + cell_idx + 1;
                        increment_cell(matrix.get_mut(right_bottom));
                    }
                }

                if row_idx > 0 {
                    let top = (row_idx - 1) * w + cell_idx;
                    increment_cell(matrix.get_mut(top));
                }

                if row_idx < l {
                    let bottom = (row_idx + 1) * w + cell_idx;
                    increment_cell(matrix.get_mut(bottom));
                }
            }
        }
    }

    // println!("{:?}", matrix);

    for row in 0..l {
        for col in 0..w {
            let cell = matrix[row * w + col];
            if cell == 0 {
                annotated[row] += " ";
            } else if cell == u8::MAX {
                annotated[row] += "*";
            } else {
                annotated[row] += &cell.to_string();
            }
        }
    }

    println!("{:?}", annotated);

    annotated
}

fn increment_cell(cell: Option<&mut u8>) {
    if let Some(v) = cell {
        if *v != u8::MAX {
            *v += 1;
        }
    }
}