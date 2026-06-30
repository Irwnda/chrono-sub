use crate::subtitle::prompt;
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) enum Encoding {
    Utf8,
    Utf16Le,
    Windows1252,
}

pub(crate) fn read_subtitle_file(
    file: &PathBuf,
) -> Result<(String, Encoding), Box<dyn std::error::Error>> {
    let bytes = fs::read(file)?;

    if bytes.starts_with(&[0xFF, 0xFE]) {
        let utf16: Vec<u16> = bytes[2..]
            .chunks_exact(2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]))
            .collect();
        return Ok((String::from_utf16(&utf16)?, Encoding::Utf16Le));
    }

    if let Ok(s) = std::str::from_utf8(&bytes) {
        return Ok((s.to_string(), Encoding::Utf8));
    }

    let (decoded, _, _) = encoding_rs::WINDOWS_1252.decode(&bytes);
    Ok((decoded.into_owned(), Encoding::Windows1252))
}

pub(crate) fn save_file(file: &Path, content: &[String], encoding: Encoding) {
    let location = prompt::prompt_naming(file);
    let text = content.join("\n");

    let bytes: Vec<u8> = match encoding {
        Encoding::Utf8 => text.into_bytes(),
        Encoding::Utf16Le => {
            let mut out = vec![0xFF, 0xFE]; // BOM
            for unit in text.encode_utf16() {
                out.extend_from_slice(&unit.to_le_bytes());
            }
            out
        }
        Encoding::Windows1252 => {
            let (encoded, _, _) = encoding_rs::WINDOWS_1252.encode(&text);
            encoded.into_owned()
        }
    };

    fs::write(location, bytes).unwrap();
}
