use std::{fs, fmt::Error};
use byte_unit::{Byte, ByteUnit, AdjustedByte};

fn file_size(path: &str) -> Result<u64, std::io::Error> {
    let size = fs::metadata(path)?.len();

    Ok(size)
}

fn adjust_into_mbs(bytes: u128) -> Result<AdjustedByte, Error> {
    let byte = Byte::from_bytes(bytes);
    let adjust = byte.get_adjusted_unit(ByteUnit::MB);

    Ok(adjust)
}

fn main() {
    let size = file_size("rust-action.pdf");

    match size {
        Ok(s) => {
            let size_in_mega_bytes = adjust_into_mbs(s.into());
            println!("File Size: {:?}", size_in_mega_bytes);
        }
        Err(e) => println!("Error: {:?}", e)
    }
}
