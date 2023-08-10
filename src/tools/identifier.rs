// Identifiers tools

pub fn is_identifier(s: &str) -> bool {
    s.chars().count() > 1 && s.chars().next().unwrap_or(' ') == '#'
}

pub fn parse_identifier(s: &str) -> Result<u64, ()> {
    if is_identifier(s) {
        let s_num: String = s.chars().skip(1).collect();

        let res = s_num.parse::<u64>();

        match res {
            Ok(r) => Ok(r),
            Err(_) => Err(()),
        }
    } else {
        let res = s.parse::<u64>();

        match res {
            Ok(r) => Ok(r),
            Err(_) => Err(()),
        }
    }
}

pub fn identifier_to_string(id: u64) -> String {
    "#".to_owned() + &id.to_string()
}
