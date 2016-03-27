use std::io::{stderr, Read, Write, Seek, SeekFrom};
use std::path::Path;

const TITLE_SIZE_OFFSET: u64 = 0xC;

fn error_out(e: &str) -> ! {
    writeln!(&mut stderr(), "Error: {}", e).expect(format!("Failed to print error: {}", e).as_str());
    std::process::exit(1);
}

fn main() {
    let filename = std::env::args().nth(1)
        .unwrap_or_else(|| error_out("No filename provided"));
    let path = Path::new(&filename);
    let file = std::fs::File::open(path.to_str()
        .unwrap_or_else(|| error_out("File could not be opened")));
    let mut file = file
        .unwrap_or_else(|_| error_out("File could not be opened"));
    file.seek(SeekFrom::Start(TITLE_SIZE_OFFSET))
        .unwrap_or_else(|_| error_out("File could not be read"));

    println!("Filepath\t{}", path.to_str().unwrap());

    let mut title_length;
    let mut byte = [0u8; 1];
    let _ = file.read_exact(&mut byte);
    if (byte[0] & (1 << 7)) != 0 {
        title_length = (byte[0] & !(1 << 7)) as usize;
        let _ = file.read_exact(&mut byte);
        title_length += (byte[0] << 7) as usize;
    }
    else {
        title_length = byte[0] as usize;
    }

    let title = {
        let mut buf = Vec::new();
        buf.resize(title_length, 0);
        let _ = file.read_exact(buf.as_mut_slice());
        String::from_utf16_lossy(unsafe {
            std::slice::from_raw_parts(buf.as_ptr() as *const u16, buf.len() / 2)
        })
    };

    println!("Title\t{}", title);
}
