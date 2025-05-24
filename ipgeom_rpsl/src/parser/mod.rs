use crate::Object;
use std::collections::HashMap;
use std::io::Read;

/// Result type returned by parser helper functions.
pub type ParseResult<'a, T> = Result<(Option<T>, &'a str, usize), ParseError>;

/// Parse an RPSL database and return a vector of RPSL objects.
pub fn parse_objects(input: &str) -> Result<Vec<Object>, ParseError> {
    parse_objects_iter(input).collect()
}

/// Incrementally parse objects from the input and return an iterator of results.
pub fn parse_objects_iter(input: &str) -> ObjectsIter<'_> {
    ObjectsIter {
        input,
        line_number: 1,
    }
}

/// Incrementally parse objects from a `Read` implementation.
pub fn parse_objects_read_iter<R: Read>(reader: R) -> ObjectsReadIter<R> {
    ObjectsReadIter {
        reader,
        buf: String::new(),
        done: false,
        line_number: 1,
    }
}

pub struct ObjectsReadIter<R: Read> {
    reader: R,
    buf: String,
    done: bool,
    line_number: usize,
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
            dbg!(self.buf.len(), self.done, self.line_number);

            match parse_object(&self.buf, self.done, self.line_number) {
                Ok((None, rest, lines)) => {
                    let consumed = self.buf.len() - rest.len();
                    self.buf.drain(..consumed);
                    self.line_number += lines;
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
                Ok((Some(obj), rest, lines)) => {
                    let consumed = self.buf.len() - rest.len();
                    self.buf.drain(..consumed);
                    self.line_number += lines;
                    return Some(Ok(obj));
                }
                Err(ParseError::Incomplete { line }) => {
                    if self.done {
                        return Some(Err(ParseError::Incomplete { line }));
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
    line_number: usize,
}

impl Iterator for ObjectsIter<'_> {
    type Item = Result<Object, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.input.is_empty() {
                return None;
            }
            match parse_object(self.input, true, self.line_number) {
                Ok((None, rest, lines)) => {
                    self.input = rest;
                    self.line_number += lines;
                    if rest.is_empty() {
                        return None;
                    }
                    continue;
                }
                Ok((Some(obj), rest, lines)) => {
                    self.input = rest;
                    self.line_number += lines;
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
fn parse_object(input: &str, eof: bool, start_line: usize) -> ParseResult<Object> {
    let mut rest = input;
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_key: Option<String> = None;
    let mut started = false;
    let mut terminated = false;
    let mut lines_consumed = 0usize;
    let mut line_no = start_line;

    while !rest.is_empty() {
        let (line, next, had_newline) = split_first_line(rest);
        if !had_newline && !eof {
            return Err(ParseError::Incomplete { line: line_no });
        }
        rest = next;
        let trimmed_end = line.trim_end_matches(['\r', '\n']);
        let trimmed = trimmed_end.trim();

        // Skip initial empty/comment lines before the object starts
        if !started && (trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('%'))
        {
            lines_consumed += 1;
            line_no += 1;
            continue;
        }

        if trimmed.is_empty() {
            if started {
                terminated = true;
                lines_consumed += 1;
                line_no += 1;
                break;
            } else {
                lines_consumed += 1;
                line_no += 1;
                continue;
            }
        }

        if trimmed.starts_with('#') || trimmed.starts_with('%') {
            // comment inside object, ignore
            lines_consumed += 1;
            line_no += 1;
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
            lines_consumed += 1;
            line_no += 1;
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
                return Err(ParseError::Incomplete { line: line_no });
            }
            let kind = if line.starts_with(|c: char| c.is_whitespace()) {
                MalformedLineError::UnexpectedContinuation
            } else {
                MalformedLineError::MissingColon
            };
            return Err(ParseError::MalformedLine {
                line: line_no,
                content: trimmed.to_string(),
                kind,
            });
        }

        if rest.is_empty() && !terminated && !eof {
            return Err(ParseError::Incomplete { line: line_no + 1 });
        }
        lines_consumed += 1;
        line_no += 1;
    }

    if started {
        if !terminated && !eof && rest.is_empty() {
            return Err(ParseError::Incomplete { line: line_no });
        }
        let mut obj = Object::new();
        for (k, vals) in map {
            for v in vals {
                obj.add(k.clone(), v);
            }
        }
        Ok((Some(obj), rest, lines_consumed))
    } else {
        if !eof && !rest.is_empty() {
            return Err(ParseError::Incomplete { line: line_no });
        }
        Ok((None, rest, lines_consumed))
    }
}

/// Split the input at the first newline, returning the line and remaining input.
fn split_first_line(input: &str) -> (&str, &str, bool) {
    if let Some(pos) = input.find('\n') {
        (&input[..pos], &input[pos + 1..], true)
    } else {
        (input, "", false)
    }
}

/// Detailed reason why a line could not be parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MalformedLineError {
    /// The line did not contain the expected `:` separator.
    MissingColon,
    /// A continuation line appeared without a preceding attribute.
    UnexpectedContinuation,
}

#[derive(Debug)]
pub enum ParseError {
    MalformedLine {
        line: usize,
        content: String,
        kind: MalformedLineError,
    },
    Io(std::io::Error),
    Incomplete {
        line: usize,
    },
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
        match err {
            ParseError::MalformedLine {
                line,
                kind: MalformedLineError::MissingColon,
                ..
            } => {
                assert_eq!(line, 1)
            }
            other => panic!("unexpected: {:?}", other),
        }
    }

    #[test]
    fn error_on_continuation_without_attribute() {
        let text = "  continuation line\n";
        let err = parse_objects(text).unwrap_err();
        match err {
            ParseError::MalformedLine {
                line,
                kind: MalformedLineError::UnexpectedContinuation,
                ..
            } => {
                assert_eq!(line, 1)
            }
            other => panic!("unexpected: {:?}", other),
        }
    }

    #[test]
    fn error_after_first_object() {
        let text = "inetnum: 1.1.1.0 - 1.1.1.255\nnetname: NET\n\ninvalid\n";
        let err = parse_objects(text).unwrap_err();
        match err {
            ParseError::MalformedLine {
                line,
                kind: MalformedLineError::MissingColon,
                ..
            } => {
                assert_eq!(line, 4)
            }
            other => panic!("unexpected: {:?}", other),
        }
    }

    #[test]
    fn incomplete_object_requires_more_input() {
        let text = "inetnum: 1.1.1.0 - 1.1.1.255\nnetname: NET";
        match parse_object(text, false, 1) {
            Err(ParseError::Incomplete { line }) => assert_eq!(line, 2),
            other => panic!("unexpected: {:?}", other),
        }
    }

    #[test]
    fn read_iter_single_object() {
        let text = "inetnum: 192.0.2.0 - 192.0.2.255\nnetname: TEST\n\n";
        let reader = std::io::Cursor::new(text);
        let objs: Vec<_> = parse_objects_read_iter(reader)
            .map(Result::unwrap)
            .collect();
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
        let reader = ChunkReader {
            inner: cur,
            chunk: 4,
        };
        let objs: Vec<_> = parse_objects_read_iter(reader)
            .map(Result::unwrap)
            .collect();
        assert_eq!(objs.len(), 2);
    }

    struct FailReader;

    impl std::io::Read for FailReader {
        fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("fail"))
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

    #[test]
    fn test_parse_long_object() {
        let text = r#"
aut-num:        AS1126
as-name:        VANCIS
descr:          Vancis Advanced ICT Services
import:         from AS42
                action pref=100;
                accept AS42
import:         from AS112
                action pref=100;
                accept AS112
import:         from AS714
                action pref=100;
                accept AS714
import:         from AS1103
                action pref=100;
                accept AS-SURFNET
import:         from AS1140
                action pref=100;
                accept AS1140
import:         from AS1200
                action pref=100;
                accept AS1200
import:         from AS1267
                action pref=100;
                accept AS1267
import:         from AS2119
                action pref=100;
                accept AS2119
import:         from AS2603
                action pref=100;
                accept AS2603
import:         from AS2611
                action pref=100;
                accept AS2611
import:         from AS3209
                action pref=100;
                accept AS3209
import:         from AS3214
                action pref=100;
                accept AS3214
import:         from AS3262
                action pref=100;
                accept AS3262
import:         from AS3265
                action pref=100;
                accept AS3265
import:         from AS3303
                action pref=100;
                accept AS3303
import:         from AS3333
                action pref=100;
                accept AS3333
import:         from AS3549
                action pref=100;
                accept ANY AND NOT {0.0.0.0/0}
import:         from AS3856
                action pref=100;
                accept AS3856
import:         from AS9583
                action pref=100;
                accept AS9583
import:         from AS13786
                action pref=100;
                accept AS13786
import:         from AS15133
                action pref=100;
                accept AS15133
import:         from AS15169
                action pref=100;
                accept AS15169
import:         from AS15879
                action pref=100;
                accept AS15879
import:         from AS12041
                action pref=100;
                accept AS12041
import:         from AS15958
                action pref=100;
                accept AS15958
import:         from AS29791
                action pref=100;
                accept AS29791
import:         from AS16243
                action pref=100;
                accept AS16243
import:         from AS5390
                action pref=100;
                accept AS5390
import:         from AS5410
                action pref=100;
                accept AS5410
import:         from AS5413
                action pref=100;
                accept AS5413
import:         from AS5524
                action pref=100;
                accept AS5524
import:         from AS6461
                action pref=100;
                accept AS6461
import:         from AS6507
                action pref=100;
                accept AS6507
import:         from AS6661
                action pref=100;
                accept AS6661
import:         from AS6667
                action pref=100;
                accept AS6667
import:         from AS6730
                action pref=100;
                accept AS6730
import:         from AS6735
                action pref=100;
                accept AS6735
import:         from AS6774
                action pref=100;
                accept AS6774
import:         from AS6777
                action pref=100;
                accept ANY
import:         from AS6830
                action pref=100;
                accept AS6830
import:         from AS6939
                action pref=100;
                accept AS6939
import:         from AS8075
                action pref=100;
                accept AS8075
import:         from AS8218
                action pref=100;
                accept AS8218
import:         from AS8315
                action pref=100;
                accept AS8315
import:         from AS8359
                action pref=100;
                accept AS8359
import:         from AS8365
                action pref=100;
                accept AS8365
import:         from AS8422
                action pref=100;
                accept AS8422
import:         from AS8426
                action pref=100;
                accept AS8426
import:         from AS8447
                action pref=100;
                accept AS8447
import:         from AS8468
                action pref=100;
                accept AS8468
import:         from AS8608
                action pref=100;
                accept AS8608
import:         from AS8657
                action pref=100;
                accept AS8657
import:         from AS8708
                action pref=100;
                accept AS8708
import:         from AS8918
                action pref=100;
                accept AS8918
import:         from AS8966
                action pref=100;
                accept AS8966
import:         from AS9002
                action pref=100;
                accept AS9002
import:         from AS9031
                action pref=100;
                accept AS9031
import:         from AS9145
                action pref=100;
                accept AS9145
import:         from AS9150
                action pref=100;
                accept AS9150
import:         from AS10310
                action pref=100;
                accept AS10310
import:         from AS12322
                action pref=100;
                accept AS12322
import:         from AS12350
                action pref=100;
                accept AS12350
import:         from AS12399
                action pref=100;
                accept AS12399
import:         from AS12414
                action pref=100;
                accept AS12414
import:         from AS12496
                action pref=100;
                accept AS12496
import:         from AS12552
                action pref=100;
                accept AS12552
import:         from AS12654
                action pref=100;
                accept AS12654
import:         from AS12713
                action pref=100;
                accept AS12713
import:         from AS12759
                action pref=100;
                accept AS12759
import:         from AS12859
                action pref=100;
                accept AS12859
import:         from AS12871
                action pref=100;
                accept AS12871
import:         from AS12902
                action pref=100;
                accept AS12902
import:         from AS13030
                action pref=100;
                accept AS13030
import:         from AS13101
                action pref=100;
                accept AS13101
import:         from AS13213
                action pref=100;
                accept AS13213
import:         from AS13237
                action pref=100;
                accept AS13237
import:         from AS13285
                action pref=100;
                accept AS13285
import:         from AS13335
                action pref=100;
                accept AS13335
import:         from AS13768
                action pref=100;
                accept AS13768
import:         from AS14907
                action pref=100;
                accept AS-WIKIMEDIA
import:         from AS15435
                action pref=100;
                accept AS15435
import:         from AS15557
                action pref=100;
                accept AS15557
import:         from AS15600
                action pref=100;
                accept AS15600
import:         from AS15703
                action pref=100;
                accept AS15703
import:         from AS15830
                action pref=100;
                accept AS15830
import:         from AS16265
                action pref=100;
                accept AS16265
import:         from AS16276
                action pref=100;
                accept AS16276
import:         from AS16298
                action pref=100;
                accept AS16298
import:         from AS16509
                action pref=100;
                accept AS16509
import:         from AS17451
                action pref=100;
                accept AS17451
import:         from AS20495
                action pref=100;
                accept AS20495
import:         from AS20504
                action pref=100;
                accept AS20504
import:         from AS20562
                action pref=100;
                accept AS20562
import:         from AS20932
                action pref=100;
                accept AS20932
import:         from AS20940
                action pref=100;
                accept AS20940
import:         from AS20953
                action pref=100;
                accept AS20953
import:         from AS20969
                action pref=100;
                accept AS20969
import:         from AS21478
                action pref=100;
                accept AS21478
import:         from AS22822
                action pref=100;
                accept AS22822
import:         from AS24167
                action pref=100;
                accept AS24167
import:         from AS24586
                action pref=100;
                accept AS24586
import:         from AS24785
                action pref=100;
                accept ANY AND NOT {0.0.0.0/0}
import:         from AS24875
                action pref=100;
                accept AS24875
import:         from AS25151
                action pref=100;
                accept AS25151
import:         from AS25152
                action pref=100;
                accept AS25152
import:         from AS25160
                action pref=100;
                accept AS25160
import:         from AS25182
                action pref=100;
                accept AS25182
import:         from AS25459
                action pref=100;
                accept AS25459
import:         from AS25542
                action pref=100;
                accept AS25542
import:         from AS25596
                action pref=100;
                accept AS25596
import:         from AS28788
                action pref=100;
                accept AS28788
import:         from AS29017
                action pref=100;
                accept AS29017
import:         from AS29208
                action pref=100;
                accept AS29208
import:         from AS29263
                action pref=100;
                accept AS29263
import:         from AS29396
                action pref=100;
                accept AS29396
import:         from AS30870
                action pref=100;
                accept AS30870
import:         from AS30132
                action pref=100;
                accept AS30132
import:         from AS30781
                action pref=100;
                accept AS30781
import:         from AS30889
                action pref=100;
                accept AS30889
import:         from AS31019
                action pref=100;
                accept AS31019
import:         from AS31027
                action pref=100;
                accept AS31027
import:         from AS31216
                action pref=100;
                accept AS31216
import:         from AS31383
                action pref=100;
                accept AS31383
import:         from AS31477
                action pref=100;
                accept AS31477
import:         from AS31500
                action pref=100;
                accept AS31500
import:         from AS31529
                action pref=100;
                accept AS31529
import:         from AS32934
                action pref=100;
                accept AS32934
import:         from AS34141
                action pref=100;
                accept AS34141
import:         from AS34307
                action pref=100;
                accept AS34307
import:         from AS34655
                action pref=100;
                accept AS34655
import:         from AS34968
                action pref=100;
                accept AS34968
import:         from AS35332
                action pref=100;
                accept AS35332
import:         from AS37100
                action pref=100;
                accept AS37100
import:         from AS39326
                action pref=100;
                accept AS39326
import:         from AS39637
                action pref=100;
                accept AS39637
import:         from AS41692
                action pref=100;
                accept AS41692
import:         from AS46489
                action pref=100;
                accept AS46489
import:         from AS47886
                action pref=100;
                accept AS47886
import:         from AS50295
                action pref=100;
                accept AS50295
import:         from AS51088
                action pref=100;
                accept AS51088
import:         from AS57976
                action pref=100;
                accept AS57976
import:         from AS58952
                action pref=100;
                accept AS58952
import:         from AS64050
                action pref=100;
                accept AS64050
import:         from AS198792
                action pref=100;
                accept AS198792
import:         from AS200020
                action pref=100;
                accept AS200020
import:         from AS206776
                action pref=100;
                accept as-histate
export:         to AS42
                announce RS-VANCIS-ROUTESET
export:         to AS112
                announce RS-VANCIS-ROUTESET
export:         to AS714
                announce RS-VANCIS-ROUTESET
export:         to AS1103
                announce RS-VANCIS-ROUTESET
export:         to AS1140
                announce RS-VANCIS-ROUTESET
export:         to AS1200
                announce RS-VANCIS-ROUTESET
export:         to AS1267
                announce RS-VANCIS-ROUTESET
export:         to AS2119
                announce RS-VANCIS-ROUTESET
export:         to AS2603
                announce RS-VANCIS-ROUTESET
export:         to AS2611
                announce RS-VANCIS-ROUTESET
export:         to AS3209
                announce RS-VANCIS-ROUTESET
export:         to AS3214
                announce RS-VANCIS-ROUTESET
export:         to AS3262
                announce RS-VANCIS-ROUTESET
export:         to AS3265
                announce RS-VANCIS-ROUTESET
export:         to AS3303
                announce RS-VANCIS-ROUTESET
export:         to AS3333
                announce RS-VANCIS-ROUTESET
export:         to AS3549
                announce RS-VANCIS-ROUTESET
export:         to AS3856
                announce RS-VANCIS-ROUTESET
export:         to AS4651
                announce RS-VANCIS-ROUTESET
export:         to AS9583
                announce RS-VANCIS-ROUTESET
export:         to AS29791
                announce RS-VANCIS-ROUTESET
export:         to AS5390
                announce RS-VANCIS-ROUTESET
export:         to AS5410
                announce RS-VANCIS-ROUTESET
export:         to AS5413
                announce RS-VANCIS-ROUTESET
export:         to AS5524
                announce RS-VANCIS-ROUTESET
export:         to AS6461
                announce RS-VANCIS-ROUTESET
export:         to AS6507
                announce RS-VANCIS-ROUTESET
export:         to AS6661
                announce RS-VANCIS-ROUTESET
export:         to AS6667
                announce RS-VANCIS-ROUTESET
export:         to AS6730
                announce RS-VANCIS-ROUTESET
export:         to AS6735
                announce RS-VANCIS-ROUTESET
export:         to AS6774
                announce RS-VANCIS-ROUTESET
export:         to AS6777
                action community .= { 6777:6777 };
                announce RS-VANCIS-ROUTESET
export:         to AS6830
                announce RS-VANCIS-ROUTESET
export:         to AS6939
                announce RS-VANCIS-ROUTESET
export:         to AS8075
                announce RS-VANCIS-ROUTESET
export:         to AS8218
                announce RS-VANCIS-ROUTESET
export:         to AS8315
                announce RS-VANCIS-ROUTESET
export:         to AS8359
                announce RS-VANCIS-ROUTESET
export:         to AS8365
                announce RS-VANCIS-ROUTESET
export:         to AS8422
                announce RS-VANCIS-ROUTESET
export:         to AS8426
                announce RS-VANCIS-ROUTESET
export:         to AS8447
                announce RS-VANCIS-ROUTESET
export:         to AS8468
                announce RS-VANCIS-ROUTESET
export:         to AS8608
                announce RS-VANCIS-ROUTESET
export:         to AS8657
                announce RS-VANCIS-ROUTESET
export:         to AS8708
                announce RS-VANCIS-ROUTESET
export:         to AS8918
                announce RS-VANCIS-ROUTESET
export:         to AS8966
                announce RS-VANCIS-ROUTESET
export:         to AS9002
                announce RS-VANCIS-ROUTESET
export:         to AS9031
                announce RS-VANCIS-ROUTESET
export:         to AS9145
                announce RS-VANCIS-ROUTESET
export:         to AS9150
                announce RS-VANCIS-ROUTESET
export:         to AS10310
                announce RS-VANCIS-ROUTESET
export:         to AS12041
                announce RS-VANCIS-ROUTESET
export:         to AS12322
                announce RS-VANCIS-ROUTESET
export:         to AS12350
                announce RS-VANCIS-ROUTESET
export:         to AS12399
                announce RS-VANCIS-ROUTESET
export:         to AS12414
                announce RS-VANCIS-ROUTESET
export:         to AS12552
                announce RS-VANCIS-ROUTESET
export:         to AS12496
                announce RS-VANCIS-ROUTESET
export:         to AS12654
                announce RS-VANCIS-ROUTESET
export:         to AS12713
                announce RS-VANCIS-ROUTESET
export:         to AS12759
                announce RS-VANCIS-ROUTESET
export:         to AS12859
                announce RS-VANCIS-ROUTESET
export:         to AS12871
                announce RS-VANCIS-ROUTESET
export:         to AS12902
                announce RS-VANCIS-ROUTESET
export:         to AS13030
                announce RS-VANCIS-ROUTESET
export:         to AS13101
                announce RS-VANCIS-ROUTESET
export:         to AS13213
                announce RS-VANCIS-ROUTESET
export:         to AS13237
                announce RS-VANCIS-ROUTESET
export:         to AS13285
                announce RS-VANCIS-ROUTESET
export:         to AS13335
                announce RS-VANCIS-ROUTESET
export:         to AS13768
                announce RS-VANCIS-ROUTESET
export:         to AS13786
                announce RS-VANCIS-ROUTESET
export:         to AS14907
                announce RS-VANCIS-ROUTESET
export:         to AS15435
                announce RS-VANCIS-ROUTESET
export:         to AS15557
                announce RS-VANCIS-ROUTESET
export:         to AS15600
                announce RS-VANCIS-ROUTESET
export:         to AS15703
                announce RS-VANCIS-ROUTESET
export:         to AS15133
                announce RS-VANCIS-ROUTESET
export:         to AS15169
                announce RS-VANCIS-ROUTESET
export:         to AS15830
                announce RS-VANCIS-ROUTESET
export:         to AS15879
                announce RS-VANCIS-ROUTESET
export:         to AS15958
                announce RS-VANCIS-ROUTESET
export:         to AS16243
                announce RS-VANCIS-ROUTESET
export:         to AS16265
                announce RS-VANCIS-ROUTESET
export:         to AS16276
                announce RS-VANCIS-ROUTESET
export:         to AS16298
                announce RS-VANCIS-ROUTESET
export:         to AS16509
                announce RS-VANCIS-ROUTESET
export:         to AS17451
                announce RS-VANCIS-ROUTESET
export:         to AS20495
                announce RS-VANCIS-ROUTESET
export:         to AS20504
                announce RS-VANCIS-ROUTESET
export:         to AS20562
                announce RS-VANCIS-ROUTESET
export:         to AS20932
                announce RS-VANCIS-ROUTESET
export:         to AS20940
                announce RS-VANCIS-ROUTESET
export:         to AS20953
                announce RS-VANCIS-ROUTESET
export:         to AS20969
                announce RS-VANCIS-ROUTESET
export:         to AS21478
                announce RS-VANCIS-ROUTESET
export:         to AS22822
                announce RS-VANCIS-ROUTESET
export:         to AS24167
                announce RS-VANCIS-ROUTESET
export:         to AS24586
                announce RS-VANCIS-ROUTESET
export:         to AS24785
                announce RS-VANCIS-ROUTESET
export:         to AS24875
                announce RS-VANCIS-ROUTESET
export:         to AS25151
                announce RS-VANCIS-ROUTESET
export:         to AS25152
                announce RS-VANCIS-ROUTESET
export:         to AS25160
                announce RS-VANCIS-ROUTESET
export:         to AS25182
                announce RS-VANCIS-ROUTESET
export:         to AS25459
                announce RS-VANCIS-ROUTESET
export:         to AS25542
                announce RS-VANCIS-ROUTESET
export:         to AS25596
                announce RS-VANCIS-ROUTESET
export:         to AS28788
                announce RS-VANCIS-ROUTESET
export:         to AS29017
                announce RS-VANCIS-ROUTESET
export:         to AS29208
                announce RS-VANCIS-ROUTESET
export:         to AS29263
                announce RS-VANCIS-ROUTESET
export:         to AS29396
                announce RS-VANCIS-ROUTESET
export:         to AS30132
                announce RS-VANCIS-ROUTESET
export:         to AS30781
                announce RS-VANCIS-ROUTESET
export:         to AS30870
                announce RS-VANCIS-ROUTESET
export:         to AS30889
                announce RS-VANCIS-ROUTESET
export:         to AS31019
                announce RS-VANCIS-ROUTESET
export:         to AS31027
                announce RS-VANCIS-ROUTESET
export:         to AS31216
                announce RS-VANCIS-ROUTESET
export:         to AS31383
                announce RS-VANCIS-ROUTESET
export:         to AS31477
                announce RS-VANCIS-ROUTESET
export:         to AS31500
                announce RS-VANCIS-ROUTESET
export:         to AS31529
                announce RS-VANCIS-ROUTESET
export:         to AS32934
                announce RS-VANCIS-ROUTESET
export:         to AS34141
                announce RS-VANCIS-ROUTESET
export:         to AS34307
                announce RS-VANCIS-ROUTESET
export:         to AS34655
                announce RS-VANCIS-ROUTESET
export:         to AS34968
                announce RS-VANCIS-ROUTESET
export:         to AS35332
                announce RS-VANCIS-ROUTESET
export:         to AS37100
                announce RS-VANCIS-ROUTESET
export:         to AS39326
                announce RS-VANCIS-ROUTESET
export:         to AS39637
                announce RS-VANCIS-ROUTESET
export:         to AS41692
                announce RS-VANCIS-ROUTESET
export:         to AS46489
                announce RS-VANCIS-ROUTESET
export:         to AS47886
                announce RS-VANCIS-ROUTESET
export:         to AS50295
                announce RS-VANCIS-ROUTESET
export:         to AS57976
                announce RS-VANCIS-ROUTESET
export:         to AS58952
                announce RS-VANCIS-ROUTESET
export:         to AS51088
                announce RS-VANCIS-ROUTESET
export:         to AS64050
                announce RS-VANCIS-ROUTESET
export:         to AS198792
                announce RS-VANCIS-ROUTESET
export:         to AS200020
                announce RS-VANCIS-ROUTESET
export:         to AS206776
                announce RS-VANCIS-ROUTESET
admin-c:        DUMY-RIPE
tech-c:         DUMY-RIPE
status:         LEGACY
mnt-by:         VANCIS-LIR-MNT
created:        2002-02-13T09:26:40Z
last-modified:  2023-07-10T14:29:40Z
source:         RIPE
remarks:        ****************************
remarks:        * THIS OBJECT IS MODIFIED
remarks:        * Please note that all data that is generally regarded as personal
remarks:        * data has been removed from this object.
remarks:        * To view the original object, please query the RIPE Database at:
remarks:        * http://www.ripe.net/whois
remarks:        ****************************
"#;
        let objs = parse_objects(text).unwrap();
        assert_eq!(objs.len(), 1);
        let obj = &objs[0];

        assert_eq!(obj.get("aut-num").unwrap(), ["AS1126"]);
    }
}
