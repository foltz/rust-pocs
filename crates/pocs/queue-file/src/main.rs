use queue_file::QueueFile;

fn main() {
    let mut qf = QueueFile::open("example.qf")
        .expect("cannot open queue file");

    qf.add("ELEMENT #1".as_bytes()).expect("add failed");
    qf.add("ELEMENT #2".as_bytes()).expect("add failed");
    qf.add("ELEMENT #3".as_bytes()).expect("add failed");

    qf.remove().expect("remove failed");

    for (index, elem) in qf.iter().enumerate() {
        println!(
            "{}: {} bytes -> {}",
            index,
            elem.len(),
            std::str::from_utf8(&elem).unwrap_or("<invalid>")
        );
    }

    qf.clear().expect("clear failed");
}