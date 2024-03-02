// - QueueFile wrapper with internal mutability:

use std::error::Error;
use std::future::Future;
use std::sync::{Arc, RwLock};
use queue_file::{Iter, QueueFile};

pub struct QueueWrapper {
    _filepath: String,
    _writer: RwLock<QueueFile>,
}

impl QueueWrapper {

    pub fn new(filepath: &str) -> Self {

        let writer = QueueFile::open(filepath)
            .expect("cannot open queue file");

        Self {
            _filepath: filepath.to_string(),
            _writer: RwLock::new(writer)
        }
    }

    pub fn write(&self, buf: &[u8]) {
        let _ = (*self._writer.write().unwrap()).add(buf);
    }

    pub fn read(&self) -> Result<Option<Box<[u8]>>, Box<dyn Error>> {
        match (*self._writer.write().unwrap()).peek() {
            Ok(d) => Ok(d),
            Err(e) => Err(Box::new(e)),
        }
    }
    pub fn remove(&self) {
        let _ = (*self._writer.write().unwrap()).remove();
    }
    pub fn clear(&self) {
        let _ = (*self._writer.write().unwrap()).clear();
    }


    pub fn get_reader(&self) -> Result<QueueFile, Box<dyn Error>> {
        match QueueFile::open(&self._filepath) {
            Ok(q) => Ok(q),
            Err(e) => Err(Box::new(e)),
        }
    }

    // pub async fn read_queue<F, Fut>(&self, on_event: F) //-> SendableResult<()>
    //     where F: Fn(String) -> Fut,
    //           Fut: Future<Output = ()>
    // {
    //     for (index, elem) in self._writer.iter().enumerate() {
    //
    //         let msg = std::str::from_utf8(&elem).unwrap_or("<invalid>");
    //         println!(
    //             "{}: {} bytes -> {}",
    //             index,
    //             elem.len(),
    //             msg
    //         );
    //         on_event(msg.to_string().clone()).await;
    //     }
    // }

    // pub fn get_reader(&self) -> Result<QueueFile, Box<dyn Error>> {
    //     QueueFile::open(&self._filepath).into()
    // }
}