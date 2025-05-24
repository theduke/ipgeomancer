use crate::Object;
use std::collections::HashMap;

/// Result type returned by parser helper functions.
pub type ParseResult<'a, T> = Result<(Option<T>, &'a str), ParseError>;

/// Parse an RPSL database and return a vector of RPSL objects.
pub fn parse_objects(input: &str) -> Result<Vec<Object>, ParseError> {
    parse_objects_iter(input).collect()
}

/// Incrementally parse objects from the input and return an iterator of results.
pub fn parse_objects_iter<'a>(input: &'a str) -> ObjectsIter<'a> {
    ObjectsIter { input }
}

pub struct ObjectsIter<'a> {
    input: &'a str,
}

impl<'a> Iterator for ObjectsIter<'a> {
    type Item = Result<Object, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.input.is_empty() {
                return None;
            }
            match parse_object(self.input) {
                Ok((None, rest)) => {
                    self.input = rest;
                    if rest.is_empty() {
                        return None;
                    }
                    continue;
                }
                Ok((Some(obj), rest)) => {
                    self.input = rest;
                    return Some(Ok(obj));
                }
                Err(e) => {
                    self.input = "";
                    return Some(Err(e));
                }
            }
        }
    }
}

/// Parse a single object from the input string.
fn parse_object(input: &str) -> ParseResult<Object> {
    let mut rest = input;
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_key: Option<String> = None;
    let mut started = false;

    while !rest.is_empty() {
        let (line, next) = split_first_line(rest);
        rest = next;
        let trimmed_end = line.trim_end_matches(['\r', '\n']);
        let trimmed = trimmed_end.trim();

        // Skip initial empty/comment lines before the object starts
        if !started && (trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('%'))
        {
            continue;
        }

        if trimmed.is_empty() {
            if started {
                break;
            } else {
                continue;
            }
        }

        if trimmed.starts_with('#') || trimmed.starts_with('%') {
            // comment inside object, ignore
            continue;
        }

        started = true;

        if line.starts_with(|c: char| c.is_whitespace()) && current_key.is_some() {
            if let Some(key) = &current_key {
                if let Some(values) = map.get_mut(key) {
                    if let Some(last) = values.last_mut() {
                        if !last.is_empty() && !last.ends_with(' ') && !trimmed.is_empty() {
                            last.push(' ');
                        }
                        last.push_str(trimmed);
                    } else {
                        values.push(trimmed.to_string());
                    }
                }
            }
            continue;
        }

        if let Some(pos) = trimmed.find(':') {
            let key = trimmed[..pos].trim().to_lowercase();
            let value = trimmed[pos + 1..].trim().to_string();
            map.entry(key.clone()).or_insert_with(Vec::new).push(value);
            current_key = Some(key);
        } else if let Some(key) = &current_key {
            if let Some(values) = map.get_mut(key) {
                if let Some(last) = values.last_mut() {
                    if !last.is_empty() && !last.ends_with(' ') && !trimmed.is_empty() {
                        last.push(' ');
                    }
                    last.push_str(trimmed);
                } else {
                    values.push(trimmed.to_string());
                }
            }
        } else {
            return Err(ParseError::MalformedLine(trimmed.to_string()));
        }
    }

    if started {
        let mut obj = Object::new();
        for (k, vals) in map {
            for v in vals {
                obj.add(k.clone(), v);
            }
        }
        Ok((Some(obj), rest))
    } else {
        Ok((None, rest))
    }
}

/// Split the input at the first newline, returning the line and remaining input.
fn split_first_line(input: &str) -> (&str, &str) {
    if let Some(pos) = input.find('\n') {
        (&input[..pos], &input[pos + 1..])
    } else {
        (input, "")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    MalformedLine(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_object() {
        let text = "inetnum: 192.0.2.0 - 192.0.2.255\nnetname: TEST-NET\n\n";
        let objs = parse_objects(text).unwrap();
        assert_eq!(objs.len(), 1);
        let obj = &objs[0];
        assert_eq!(obj.get("inetnum").unwrap(), ["192.0.2.0 - 192.0.2.255"]);
        assert_eq!(obj.get("netname").unwrap(), ["TEST-NET"]);
    }

    #[test]
    fn parse_multiple_objects() {
        let text = "person: John Doe\nsource: TEST\n\naut-num: AS1\nsource: TEST\n";
        let objs: Vec<_> = parse_objects(text).unwrap();
        assert_eq!(objs.len(), 2);
        assert_eq!(objs[0].get("person").unwrap(), ["John Doe"]);
        assert_eq!(objs[1].get("aut-num").unwrap(), ["AS1"]);
    }

    #[test]
    fn continuation_and_multivalue() {
        let text = "descr: First line\n  second line\nremarks: a\nremarks: b\n\n";
        let objs = parse_objects(text).unwrap();
        let obj = &objs[0];
        assert_eq!(obj.get("descr").unwrap(), ["First line second line"]);
        assert_eq!(obj.get("remarks").unwrap(), ["a", "b"]);
    }

    #[test]
    fn comments_ignored() {
        let text =
            "# comment before\ninetnum: 1.1.1.0 - 1.1.1.255\n% another comment\nnetname: NET\n\n";
        let objs = parse_objects(text).unwrap();
        assert_eq!(objs.len(), 1);
        let obj = &objs[0];
        assert_eq!(obj.get("netname").unwrap(), ["NET"]);
    }

    #[test]
    fn eof_without_trailing_newline() {
        let text = "inetnum: 192.0.2.0 - 192.0.2.255\nnetname: TEST-NET";
        let objs = parse_objects(text).unwrap();
        assert_eq!(objs.len(), 1);
        let obj = &objs[0];
        assert_eq!(obj.get("netname").unwrap(), ["TEST-NET"]);
    }

    #[test]
    fn crlf_line_endings() {
        let text = "inetnum: 192.0.2.0 - 192.0.2.255\r\nnetname: TEST-NET\r\n\r\n";
        let objs = parse_objects(text).unwrap();
        assert_eq!(objs.len(), 1);
        assert_eq!(objs[0].get("inetnum").unwrap(), ["192.0.2.0 - 192.0.2.255"]);
    }

    #[test]
    fn iterates_multiple_objects() {
        let text = "inetnum: 1.1.1.0 - 1.1.1.255\nnetname: NET1\n\ninetnum: 2.2.2.0 - 2.2.2.255\nnetname: NET2\n\n";
        let mut iter = parse_objects_iter(text);
        let obj1 = iter.next().unwrap().unwrap();
        let obj2 = iter.next().unwrap().unwrap();
        assert!(iter.next().is_none());
        assert_eq!(obj1.get("netname").unwrap(), ["NET1"]);
        assert_eq!(obj2.get("netname").unwrap(), ["NET2"]);
    }

    #[test]
    fn error_on_missing_colon() {
        let text = "inetnum 1.1.1.0 - 1.1.1.255\n";
        let err = parse_objects(text).unwrap_err();
        assert!(matches!(err, ParseError::MalformedLine(_)));
    }

    #[test]
    fn error_on_continuation_without_attribute() {
        let text = "  continuation line\n";
        let err = parse_objects(text).unwrap_err();
        assert!(matches!(err, ParseError::MalformedLine(_)));
    }
}
