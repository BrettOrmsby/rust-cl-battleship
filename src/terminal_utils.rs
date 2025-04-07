use console::{Alignment, Style, Term, style};

/// Center aligns text and prints it in the terminal
pub fn print_center(term: &Term, string: &str) {
    let (_, width) = term.size();
    string.split("\n").for_each(|line| {
        term.write_line(&console::pad_str(
            line,
            width.into(),
            Alignment::Center,
            None,
        ));
    });
}

/// Joins two strings horizontally. Will truncate the second string to match the size of the first.
pub fn join(first: String, second: String, space: usize) -> String {
    let mut second_lines = second.split("\n");
    first
        .split("\n")
        .map(|line| {
            if let Some(join_line) = second_lines.next() {
                format!("{}{}{}\n", line, " ".repeat(space), join_line)
            } else {
                line.to_string() + "\n"
            }
        })
        .collect()
}

/// Creates a coloured Battle Ship grid
pub fn create_colored_grid(colours: &[Vec<Style>]) -> String {
    let mut grid = style(" ".repeat(44))
        //.on_black()
        .to_string()
        + "\n"
        + &style("    1   2   3   4   5   6   7   8   9   10  ")
            .bold()
            // .on_black()
            .to_string()
        + "\n";
    colours.iter().enumerate().for_each(|(i, row)| {
        (0..2).for_each(|k| {
            let first_char = if k == 0 {
                std::char::from_u32('A' as u32 + i as u32).unwrap_or('a')
            } else {
                ' '
            };

            let mut line = style(format!(" {} ", first_char))
                .bold()
                // .on_black()
                .to_string();
            row.iter().for_each(|style| {
                let styled_block = style.apply_to("██");
                line += &format!("{}{}", styled_block, styled_block);
            });
            line += "\n";
            grid += &line;
        });
    });
    grid
}
