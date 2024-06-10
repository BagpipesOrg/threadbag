// decode typescript encoded diagram data

use base64::prelude::*;
use flate2::read::ZlibDecoder;
//use flate2::write::ZlibEncoder;
//use flate2::Compression;

//use std::error::Error;
use std::io::Read;
//use std::io::Write;

// this does not produce the same output as the .ts one, something to do with padding/encoding the original string to bytes
/*
pub async fn compress_string(input: &str) -> Result<String, Box<dyn Error>> {
    println!("compressing string");
    let utf8encoded = input.as_bytes();
    println!("utf8encoded: {:?}", utf8encoded);

    // Compress
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(utf8encoded).unwrap();
    let bytes = e.finish().unwrap();


    // Convert to base64
    let base64encoded = BASE64_STANDARD.encode(bytes); //encode(compressed_array);

    println!("Base64 Encoded: {}", base64encoded);

    Ok(base64encoded)
}

*/

pub async fn decompress_string(compressed_input: String) -> Result<String, String> {
    // Uncomment the following line if you want to print debugging information
    // println!("Decompressing string\nGot input: {}", compressed_input);
    // println!("f1");

    //println!("decoding bytes");
    // Decode base64 and create a Vec<u8>
    let decoded_bytes = BASE64_STANDARD.decode(compressed_input.as_bytes()).unwrap();
    //   println!("bytes decoded");

    // Decompress using zlib
    let mut decompressed = Vec::new();
    let mut decoder = ZlibDecoder::new(decoded_bytes.as_slice());
    match decoder.read_to_end(&mut decompressed) {
        Ok(_) => {
            let decompressed_str =
                String::from_utf8(decompressed).map_err(|_| "Error converting to UTF-8")?;
            //       println!("Decompressed Result: {}", decompressed_str);
            Ok(decompressed_str)
        }
        Err(_) => Err("Error decompressing string".to_string()),
    }
}
