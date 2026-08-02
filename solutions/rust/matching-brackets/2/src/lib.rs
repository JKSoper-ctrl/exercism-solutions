pub fn brackets_are_balanced(string: &str) -> bool {
    let mut opening_brackets: Vec<char> = Vec::new();
    
    for character in string.chars() {
        match character {
            '[' | '{' | '(' => opening_brackets.push(character),
            
            ']' | '}' | ')' => match opening_brackets.pop() {
                Some(inner_bracket) if brackets_match(inner_bracket, character) => {}
                _ => return false,
            },

            _ => {}
        }
    }

    opening_brackets.is_empty()
}

fn brackets_match(opening_bracket: char, closing_bracket: char) -> bool {
    matches! (
        (opening_bracket, closing_bracket),
        ('(', ')') | ('[', ']') | ('{', '}')
    )
}