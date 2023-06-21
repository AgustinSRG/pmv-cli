// Identifiers tools

pub fn is_identifier(s: &str) -> bool {
    return s.chars().count() > 1 && s.chars().nth(0).unwrap_or(' ') == '#';
}

pub fn parse_identifier(s: &str) -> Result<u64, ()> {
    if is_identifier(s) {
        let s_num: String = s.chars().skip(1).collect();

        let res = s_num.parse::<u64>();

        match res {
            Ok(r) => {
                return Ok(r);
            },
            Err(_) => {
                return Err(());
            }
        }
    } else {
        let res = s.parse::<u64>();

        match res {
            Ok(r) => {
                return Ok(r);
            },
            Err(_) => {
                return Err(());
            }
        }
    }
}

pub fn identifier_to_string(id: u64) -> String {
    return "#".to_owned() + &id.to_string();
}
