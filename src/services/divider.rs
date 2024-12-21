#![allow(unused)]

fn strip_return(s: &str) -> String {
    String::from(match s.strip_suffix('\r') {
        Some(o) => o,
        None => s
    })
}

pub fn divide_proposition(proposition: String) -> Vec<String> {
    // splits the tokens in vecs separated by new lines
    proposition.split('\n').map(strip_return).collect()
}
