use crate::Object;
use std::collections::HashMap;
use std::io::Read;

/// Result type returned by parser helper functions.
pub type ParseResult<'a, T> = Result<(Option<T>, &'a str), ParseError>;

/// Parse an RPSL database and return a vector of RPSL objects.
pub fn parse_objects(input: &str) -> Result<Vec<Object>, ParseError> {
    parse_objects_iter(input).collect()
}

/// Incrementally parse objects from the input and return an iterator of results.
pub fn parse_objects_iter(input: &str) -> ObjectsIter<'_> {
    ObjectsIter { input }
}

/// Incrementally parse objects from a `Read` implementation.
pub fn parse_objects_read_iter<R: Read>(reader: R) -> ObjectsReadIter<R> {
    ObjectsReadIter {
        reader,
        buf: String::new(),
        done: false,
    }
}

pub struct ObjectsReadIter<R: Read> {
    reader: R,
    buf: String,
    done: bool,
}

impl<R: Read> ObjectsReadIter<R> {
    fn read_more(&mut self) -> Result<(), std::io::Error> {
        let mut tmp = [0u8; 8192];
        match self.reader.read(&mut tmp) {
            Ok(0) => {
                self.done = true;
                Ok(())
            }
            Ok(n) => {
                self.buf.push_str(&String::from_utf8_lossy(&tmp[..n]));
                Ok(())
            }
            Err(e) => {
                self.done = true;
                Err(e)
            }
        }
    }
}

impl<R: Read> Iterator for ObjectsReadIter<R> {
    type Item = Result<Object, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.done && self.buf.is_empty() {
                return None;
            }

            match parse_object(&self.buf, self.done) {
                Ok((None, rest)) => {
                    self.buf = rest.to_string();
                    if self.buf.is_empty() {
                        if self.done {
                            return None;
                        }
                        if let Err(e) = self.read_more() {
                            return Some(Err(ParseError::Io(e)));
                        }
                        continue;
                    }
                    continue;
                }
                Ok((Some(obj), rest)) => {
                    self.buf = rest.to_string();
                    return Some(Ok(obj));
                }
                Err(ParseError::Incomplete) => {
                    if self.done {
                        return Some(Err(ParseError::Incomplete));
                    }
                    if let Err(e) = self.read_more() {
                        return Some(Err(ParseError::Io(e)));
                    }
                    continue;
                }
                Err(e) => {
                    self.done = true;
                    self.buf.clear();
                    return Some(Err(e));
                }
            }
        }
    }
}

pub struct ObjectsIter<'a> {
    input: &'a str,
}

impl Iterator for ObjectsIter<'_> {
    type Item = Result<Object, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.input.is_empty() {
                return None;
            }
            match parse_object(self.input, true) {
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
/// The `eof` flag indicates whether no more data will follow the input.
fn parse_object(input: &str, eof: bool) -> ParseResult<Object> {
    let mut rest = input;
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_key: Option<String> = None;
    let mut started = false;
    let mut terminated = false;

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
                terminated = true;
                break;
            } else {
                if next.is_empty() && !eof {
                    return Err(ParseError::Incomplete);
                }
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
            map.entry(key.clone()).or_default().push(value);
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
            if rest.is_empty() && !eof {
                return Err(ParseError::Incomplete);
            }
            return Err(ParseError::MalformedLine(trimmed.to_string()));
        }

        if rest.is_empty() && !terminated && !eof {
            return Err(ParseError::Incomplete);
        }
    }

    if started {
        if !terminated && !eof && rest.is_empty() {
            return Err(ParseError::Incomplete);
        }
        let mut obj = Object::new();
        for (k, vals) in map {
            for v in vals {
                obj.add(k.clone(), v);
            }
        }
        Ok((Some(obj), rest))
    } else {
        if !eof && !rest.is_empty() {
            return Err(ParseError::Incomplete);
        }
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

#[derive(Debug)]
pub enum ParseError {
    MalformedLine(String),
    Io(std::io::Error),
    Incomplete,
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

    #[test]
    fn read_iter_single_object() {
        let text = "inetnum: 192.0.2.0 - 192.0.2.255\nnetname: TEST\n\n";
        let reader = std::io::Cursor::new(text);
        let objs: Vec<_> = parse_objects_read_iter(reader).map(Result::unwrap).collect();
        assert_eq!(objs.len(), 1);
        assert_eq!(objs[0].get("netname").unwrap(), ["TEST"]);
    }

    struct ChunkReader<R: std::io::Read> {
        inner: R,
        chunk: usize,
    }

    impl<R: std::io::Read> std::io::Read for ChunkReader<R> {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let max = std::cmp::min(self.chunk, buf.len());
            self.inner.read(&mut buf[..max])
        }
    }

    #[test]
    fn read_iter_multiple_objects_small_chunks() {
        let text = "person: John\nsource: T\n\naut-num: AS1\nsource: T\n\n";
        let cur = std::io::Cursor::new(text);
        let reader = ChunkReader { inner: cur, chunk: 4 };
        let objs: Vec<_> = parse_objects_read_iter(reader).map(Result::unwrap).collect();
        assert_eq!(objs.len(), 2);
    }

    struct FailReader;

    impl std::io::Read for FailReader {
        fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "fail"))
        }
    }

    #[test]
    fn read_iter_propagates_read_error() {
        let mut iter = parse_objects_read_iter(FailReader);
        match iter.next() {
            Some(Err(ParseError::Io(e))) => {
                assert_eq!(e.kind(), std::io::ErrorKind::Other);
            }
            other => panic!("unexpected: {:?}", other),
        }
    }

}
