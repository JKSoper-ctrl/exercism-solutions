pub fn build_proverb(list: &[&str]) -> String {
    let number_in_list = list.len();
    let mut proverb = String::new();

    if number_in_list == 0 { return proverb }
    
    for line_number in 1..number_in_list {
        proverb.push_str("For want of a ");
        proverb.push_str(list[line_number - 1]);
        proverb.push_str(" the ");
        proverb.push_str(list[line_number]);
        proverb.push_str(" was lost.\n");
    }

    proverb.push_str("And all for the want of a ");
    proverb.push_str(list[0]);
    proverb.push('.');

    proverb
}
