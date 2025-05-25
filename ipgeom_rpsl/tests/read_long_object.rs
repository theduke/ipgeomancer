use ipgeom_rpsl::parse_objects_read_iter;
use std::io::Cursor;

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
fn read_long_object_in_small_chunks() {
    let text = include_str!("long_object.txt");
    let reader = ChunkReader {
        inner: Cursor::new(text),
        chunk: 16,
    };
    let objs: Vec<_> = parse_objects_read_iter(reader)
        .map(Result::unwrap)
        .collect();
    assert_eq!(objs.len(), 1);
    let obj = &objs[0];
    assert_eq!(obj.get("aut-num").unwrap(), ["AS1126"]);
}
