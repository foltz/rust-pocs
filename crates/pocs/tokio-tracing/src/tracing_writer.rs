pub struct TracingWriter;

impl std::io::Write for TracingWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let buf_len = buf.len();

        match std::str::from_utf8(buf) {
            Ok(str) => {
                println!("tw: {:?}", str);
            }
            Err(e) => {
                println!("tw: {:?}", e.to_string());

            }
        }

        Ok(buf_len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}