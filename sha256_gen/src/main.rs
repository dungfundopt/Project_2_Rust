use std::env;
use std::fs::File;
use std::io::{self, Read, BufReader};
use sha2::{Sha256, Digest};

fn main() -> io::Result<()> {
    // 1. Lấy đường dẫn file từ đối số dòng lệnh.
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return Ok(()); // Thoát chương trình, không cần báo lỗi.
    }
    let filename = &args[1];

    // 2. Mở file.
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    // 3. Tạo đối tượng băm SHA-256.
    let mut hasher = Sha256::new();

    // 4. Đọc file và cập nhật hasher.
    let mut buffer = [0; 1024]; // Đọc từng chunk 1KB.
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break; // Đã đọc hết file.
        }
        hasher.update(&buffer[..count]);
    }

    // 5. Tính toán và in ra mã băm.
    let result = hasher.finalize();
    println!("{:x}", result); // In ra dạng hexadecimal.

    Ok(())
}