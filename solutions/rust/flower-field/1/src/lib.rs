pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {return Vec::new()}
    
    let mut hinted_garden: Vec<String> = Vec::new();

    let garden_width = garden[0].len();
    let garden_height = garden.len();
    
    println!("Garden width: {garden_width}.");
    println!("Garden height: {garden_height}.");
    
    for y in 0..garden_height {
        let mut line = String::new();
        println!("y: {}", &y);
        for x in 0..garden_width {
            println!("x: {}", &x);
            if get_byte(garden, y, x) == (" ".as_bytes()[0]) {
                let mut number_of_adjacent_flowers = 0;

                for x_dif in -1isize..=1 {
                    if (x as isize) + x_dif < 0 {continue}
                    if (x as isize) + x_dif >= (garden_width as isize) {continue}
                    for y_dif in -1isize..=1 {
                        if (y as isize) + y_dif < 0 {continue}
                        if (y as isize) + y_dif >= (garden_height as isize) {continue}
                        
                        if get_byte(garden, (((y as isize) + y_dif) as usize), (((x as isize) + x_dif) as usize)) == ("*".as_bytes()[0]) {
                            number_of_adjacent_flowers = number_of_adjacent_flowers + 1;
                        }
                    }
                }

                if number_of_adjacent_flowers == 0 {
                    line.push_str(" ");
                } else {
                    line.push_str(&number_of_adjacent_flowers.to_string());
                }
                
            } else {
                line.push_str("*");
            }
        }
        println!("Line: {}", &line);
        hinted_garden.push(line);
    }

    dbg!(&hinted_garden);
    hinted_garden
}

fn get_byte<'a>(garden: &'a [&'a str], row_number: usize, collumn_number: usize) -> u8 {
    let row = garden[row_number];
    let row_bytes = row.as_bytes();
    let bytes = row_bytes[collumn_number];
    bytes
}