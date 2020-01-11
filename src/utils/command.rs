#[derive(PartialEq, Debug)] // for debug/test
pub enum Command {
    Help,
    Random(Vec<String>),
    Stamp(usize, Vec<String>),
    Omikuji,
}

fn comma_split<T: std::iter::Iterator<Item = String>>(iter: T) -> Vec<String> {
    let mut res = Vec::new();
    for i in iter {
        for c in i.split(':').filter(|x| !x.is_empty()) {
            res.push(c.to_string());
        }
    }
    res
}

pub fn parse_command(plain_text: &str) -> Option<Command> {
    use Command::*;
    let mut terms = plain_text.split_whitespace().map(|x| x.to_lowercase());
    let command = terms.next();
    if command.is_none() {
        return None;
    }

    let mut command = command.unwrap();
    if command.as_str() == "@bot_xecua_odai" {
        match terms.next() {
            Some(c) => {
                command = c;
            }
            None => {
                return None;
            }
        }
    }

    match command.as_str() {
        "/help" => Some(Help),
        "/random" => Some(Random(terms.collect())),
        "/stamp" => match terms.clone().next() {
            // clone `terms` not to consume original
            Some(s) => match s.parse::<usize>() {
                Ok(n) => Some(Stamp(n, comma_split(terms.skip(1)))),
                Err(_) => Some(Stamp(1, comma_split(terms))),
            },
            None => Some(Stamp(1, Vec::new())),
        },
        "/omikuji" => Some(Omikuji),
        _ => None,
    }
}
