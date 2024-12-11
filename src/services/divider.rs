pub fn divide_proposition(proposition: String) -> Vec<String> {
    // splits the tokens in vecs separated by new lines
    let mut vec = Vec::new();
    let mut curr: String = String::new();
    for c in proposition.chars() {
        if c == '\r' { continue; }
        if c == '\n' {
            if !curr.is_empty() {
                vec.push(curr);
                curr = String::new();
            }
            continue;
        }
        curr.push(c);
    }
    if !curr.is_empty() {
        vec.push(curr);
    }
    vec
}