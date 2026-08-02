pub fn brackets_are_balanced(string: &str) -> bool {
    let mut opening_brackets: Vec<char> = Vec::new();
    
    for character in string.chars() {
        match character {
            '[' | '{' | '(' => {
                opening_brackets.push(character);
                dbg!(&opening_brackets);
            },
            ']' | '}' | ')' => {
                if let Some(inner_bracket) = opening_brackets.last() {
                    if brackets_match(inner_bracket, &character) {
                        dbg!(opening_brackets.pop());
                    } else {
                        return false
                    }
                } else {
                    return false
                }
            },
            _ => continue,
        }
    }

    opening_brackets.is_empty()
}

fn brackets_match(opening_bracket: &char, closing_bracket: &char) -> bool {
    match opening_bracket {
        '[' => if closing_bracket == &']' {true} else {false},
        '{' => if closing_bracket == &'}' {true} else {false},
        '(' => if closing_bracket == &')' {true} else {false},
        _ => false,
    }
}