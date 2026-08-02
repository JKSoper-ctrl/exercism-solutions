pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return Vec::new();
    }

    let mut hinted_garden: Vec<String> = Vec::new();

    let garden_width = garden[0].len();
    let garden_height = garden.len();

    for y in 0..garden_height {
        let mut line = String::new();

        for x in 0..garden_width {
            if get_byte(garden, y, x) == b' ' {
                let mut number_of_adjacent_flowers = 0;

                for x_dif in -1isize..=1 {
                    for y_dif in -1isize..=1 {
                        let adjacent_x = x as isize + x_dif;
                        let adjacent_y = y as isize + y_dif;

                        if adjacent_x < 0
                            || adjacent_x >= garden_width as isize
                            || adjacent_y < 0
                            || adjacent_y >= garden_height as isize
                        {
                            continue;
                        }

                        if get_byte(
                            garden,
                            adjacent_y as usize,
                            adjacent_x as usize,
                        ) == b'*'
                        {
                            number_of_adjacent_flowers += 1;
                        }
                    }
                }

                if number_of_adjacent_flowers == 0 {
                    line.push(' ');
                } else {
                    line.push_str(&number_of_adjacent_flowers.to_string());
                }
            } else {
                line.push('*');
            }
        }

        hinted_garden.push(line);
    }

    hinted_garden
}

fn get_byte(garden: &[&str], row_number: usize, collumn_number: usize) -> u8 {
    garden[row_number].as_bytes()[collumn_number]
}