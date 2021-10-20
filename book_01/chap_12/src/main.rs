fn test_read_all() {
    let path = "sample.txt";
    if let Ok(data) = std::fs::read_to_string(path) {
        println!("{}\n{}", data.len(), data);
    } else {
        println!("Error to read");
    }

    match std::fs::read_to_string(path) {
        Ok(data) => {
            println!("{}\n{}", data.len(), data);
        },
        Err(e) => {
            println!("cannot read: {:?}", e);
        },
    }
}

fn test_read_open() {
    use std::io::Read;
    let path = "sample.txt";
    let mut file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) => return,
    };
    println!("{:?}", file);
    let mut data = String::new();
    file.read_to_string(&mut data).expect("erro to read");
    println!("{}", data);

    if let Ok(mut file) = std::fs::File::open(path) {
        let mut data = String::new();
        if let Ok(_) = file.read_to_string(&mut data) {
            println!("{}", data);
        }
    }

    let mut file = std::fs::File::open(path).expect("cannot open file");
    let mut buf: [u8; 1] = [0; 1];
    for i in 0 .. 10 {
        file.read(&mut buf).expect("Erro to read");
        println!("{:03}:  is 0x{:02X}, {}", i, buf[0], buf[0] as char);
    }

    use std::io::{BufRead, BufReader};
    let file = std::fs::File::open(path).expect("file not open");
    for line in BufReader::new(file).lines() {
        if let Ok(l) = line {
            println!("line: {}", l);
        }
    }
}

fn test_read2() -> std::io::Result<()> {
    use std::io::{BufRead, BufReader};
    let path = "sample.txt";
    let file = std::fs::File::open(path)?;
    for line in std::io::BufReader::new(file).lines() {
        println!("{}", line?);
    }
    Ok(())
}

fn test_write() {
    use std::io::Write;
    let path = "out.txt";
    let mut file = std::fs::File::create(path).expect("cannot create file");
    writeln!(file, "abc").expect("cannot write");

    file.write(b"test\n").expect("cannot write");
    file.write("test\n".as_bytes()).expect("cannot write");
    file.write(&[0x41, 0x42]).expect("cannot write");
    for it in "test\n".as_bytes() {
        file.write(&[*it]).expect("cannot write");
    }
}

use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};
use std::env;
fn do_print<R>(reader: BufReader<R>) where R: std::io::Read {
    let mut writer = BufWriter::new(std::io::stdout());
    for it in reader.bytes() {
        writer.write(&[it.unwrap()]).expect("cannot write");
    }
}

fn test_cat() {
    let args = env::args().collect::<Vec<String>>();
    println!("args = {:?}", args);
    if args.len() <= 1 {
        let reader = BufReader::new(std::io::stdin());
        do_print(reader);
    } else {
        let file = File::open(&args[1]).expect("cannot open file");
        let reader = BufReader::new(file);
        do_print(reader);
    }
}

fn main() -> std::io::Result<()>{
    // test_read_all();
    // test_read_open();
    // test_read2()?;
    // test_write();
    test_cat();
    Ok(())
}
