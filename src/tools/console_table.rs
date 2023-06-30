// Table to print into console

use std::cmp::max;

use unicode_width::{UnicodeWidthStr, UnicodeWidthChar};

const MIN_ALLOWED_COL_LENGTH: usize = 8;

pub fn to_csv_string(s: &str) -> String {
    return serde_json::to_string(s).unwrap();
}

pub fn print_table(head: &Vec<String>, body: &Vec<Vec<String>>, std_err: bool) -> () {
    let len = head.len();

    // Get term size

    let (term_cols, _) = term_size::dimensions()
        .unwrap_or((0, 0));

    // Check the max sizes
    let mut head_sizes: Vec<usize> = vec![0; len];

    for (i, head_col) in head.iter().enumerate() {
        let head_col_len = head_col.width();
        if head_sizes[i] < head_col_len {
            head_sizes[i] = head_col_len;
        }
    }

    let mut body_sizes: Vec<usize> = vec![0; len];

    for body_row in body {
        for (i, body_col) in body_row.iter().enumerate() {
            if i >= len {
                break;
            }

            let body_col_len = body_col.width();
            if body_sizes[i] < body_col_len {
                body_sizes[i] = body_col_len;
            }
        }
    }

    let mut max_sizes: Vec<usize> = vec![0; len];
    let mut total_max_size: usize = 0;

    let mut extra_size: usize = 1;

    for (i, body_size) in body_sizes.iter().enumerate() {
        extra_size += 3;

        if head_sizes[i] > *body_size {
            max_sizes[i] = head_sizes[i];
            total_max_size += head_sizes[i];
        } else {
            max_sizes[i] = *body_size;
            total_max_size += body_size;
        }
    }

    let total_size = total_max_size + extra_size;

    if term_cols >= total_size {
        print_table_with_sizes(head, body, &max_sizes, std_err);
    } else {
        let col_allowed_size = max(MIN_ALLOWED_COL_LENGTH, (term_cols - extra_size) / len);

        let mut allowed_sizes: Vec<usize> = vec![0; len];
        let mut spare_size: usize = 0;
        let mut overflow_count: usize = 0;

        // Compute spare size and set sizes less than the allowed max size

        for (i, max_size) in max_sizes.iter().enumerate() {
            if *max_size <= col_allowed_size {
                spare_size += col_allowed_size - *max_size;

                allowed_sizes[i] = *max_size;
            } else {
                overflow_count += 1;
            }
        }

        // Evenly share the spare size

        if overflow_count > 0 {
            let spare_size_split = spare_size / overflow_count;
            let new_allowed_size = col_allowed_size + spare_size_split;

            for (i, max_size) in max_sizes.iter().enumerate() {
                if *max_size > col_allowed_size {
                    allowed_sizes[i] = new_allowed_size;
                }
            }
        }

        // Print
        print_table_with_sizes(head, body, &allowed_sizes, std_err);
    }
}

fn print_table_with_sizes(head: &Vec<String>, body: &Vec<Vec<String>>, sizes: &Vec<usize>, std_err: bool) -> () {
    print_table_separator(&sizes, std_err);

    print_table_line(head, sizes, std_err);

    print_table_separator(&sizes, std_err);

    for body_row in body {
        print_table_line(body_row, sizes, std_err);
    }

    print_table_separator(&sizes, std_err);
}

fn print_table_line(col: &Vec<String>, sizes: &Vec<usize>, std_err: bool) {
    let mut more_lines = true;
    let mut line = 0;

    while more_lines {
        more_lines = false;

        let mut line_str = "".to_string();

        line_str.push('|');

        for (i, s) in sizes.iter().enumerate() {
            line_str.push(' ');

            if i < col.len() {
                let char_skip = *s * line;

                if char_skip + *s < col[i].width() {
                    more_lines = true;
                }

                line_str.push_str(&pad_str(&sub_string(&col[i], char_skip, *s), *s));
            } else {
                line_str.push_str(&pad_str("", *s));
            }

            line_str.push(' ');
            line_str.push('|');
        }

        if std_err {
            eprintln!("{line_str}");
        } else {
            println!("{line_str}");
        }

        if more_lines {
            line += 1;
        }
    }
}

fn sub_string(original_str: &str, skip: usize, limit: usize) -> String {
    if limit == 0 {
        return "".to_string();
    }
    if skip >= original_str.width() {
        return "".to_string();
    } else if skip == 0 {
        if limit >= original_str.width() {
            return original_str.to_string();
        } else {
            let mut res = "".to_string();
            let mut res_width: usize = 0;

            for c in original_str.chars() {
                let c_width = c.width_cjk().unwrap_or(0);

                if res_width + c_width > limit {
                    return res;
                }

                res.push(c);
                res_width += c_width;
            }

            return res;
        }
    } else {
        let mut skipped_str = "".to_string();
        let mut chars_count_skip: usize = 0;

        for c in original_str.chars() {
            let mut new_skipped_str = skipped_str.clone();
            new_skipped_str.push(c);

            if new_skipped_str.width() > skip {
                break;
            }

            skipped_str = new_skipped_str;
            chars_count_skip += 1;
        }

        skipped_str = original_str.chars().skip(chars_count_skip).collect();

        if limit >= skipped_str.width() {
            return skipped_str;
        } else {
            let mut res = "".to_string();

            for c in skipped_str.chars() {
                let mut new_res = res.clone();
                new_res.push(c);

                if new_res.width() > limit {
                    return res;
                }

                res = new_res;
            }

            return res;
        }
    }
}

fn print_table_separator(sizes: &Vec<usize>, std_err: bool) -> () {
    let mut line = "".to_string();

    line.push('|');

    for s in sizes {
        let padded_size = *s + 2;
        for _ in 0..padded_size {
            line.push('-');
        }

        line.push('|');
    }

    if std_err {
        eprintln!("{line}");
    } else {
        println!("{line}");
    }
}

fn pad_str(str: &str, s: usize) -> String {
    let mut res = str.to_string();

    let w = res.width();

    if w < s {
        let d = s - w;

        for _ in 0..d {
            res.push(' ');
        }
    }

    return res;
}
