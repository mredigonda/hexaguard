use inquire::Select;
use inquire::Text;
use std::path::Path;
mod file;

enum Mode {
    Encrypt,
    Decrypt,
}

fn main() {
    let options = vec!["Encrypt", "Decrypt"];

    let ans = Select::new("What?", options)
        .prompt()
        .expect("No option selected.");

    let mode = match ans {
        "Encrypt" => Mode::Encrypt,
        "Decrypt" => Mode::Decrypt,
        _ => panic!("Invalid mode"),
    };

    let base_filename = Text::new("What filename?").prompt().expect("No filename.");
    let raw_file = file::File::new(&base_filename);
    let file = get_processed_file(&raw_file, &mode);

    // If we are decrypting, we are sure that the file is hexa
    assert!(!matches!(&mode, Mode::Decrypt) || file.is_hexa());

    if !file.exists() {
        println!("No file.");
        return;
    }

    let passphrase = Text::new("Passphrase?").prompt().expect("No passphrase.");
    let passphrase2 = Text::new("Passphrase again?")
        .prompt()
        .expect("No passphrase");

    if passphrase != passphrase2 {
        println!("Passphrases wrong buddy");
        return;
    }

    // Vector of separator bytes, completely random, based on: "6b 05 8e d2 3f 67 c7 b3 a3 71 a4 12 e1 a6 fa 35"
    let mut separator_bytes = vec![
        0x6b, 0x05, 0x8e, 0xd2, 0x3f, 0x67, 0xc7, 0xb3, 0xa3, 0x71, 0xa4, 0x12, 0xe1, 0xa6, 0xfa,
        0x35,
    ];

    if matches!(mode, Mode::Encrypt) {
        let encrypted_file = file.encrypt(&passphrase);

        let hexa_filename = Text::new("Where?").prompt().expect("No result filename.");
        let hexa_file = file::File::new(&hexa_filename);

        if !hexa_file.is_hexa() {
            println!("Result filename must end in .hexa");  
            return;
        }

        let mut existing_bytes = hexa_file.get_bytes();
        let mut new_bytes = encrypted_file.get_bytes();
        let mut concatenated_bytes = Vec::new();
        concatenated_bytes.append(&mut existing_bytes);
        concatenated_bytes.append(&mut new_bytes);
        concatenated_bytes.append(&mut separator_bytes);
        // Write to file, overwriting the existing file
        hexa_file.create_with_bytes(&concatenated_bytes);
        let hex_file = hexa_file.hexa_to_hex();
        let png_file = hex_file.hex_to_qr_png();
        png_file.qr_png_to_pdf();

        encrypted_file.delete();
        hex_file.delete();
    } else {
        let hexa_bytes = file.get_bytes();
        let split_bytes: Vec<Vec<u8>> = partition_by_separator(&hexa_bytes, &separator_bytes);

        // Write each to its own file
        for i in 0..split_bytes.len() {
            let file = file::File::new(&format!("{}.gpg", i));
            file.create_with_bytes(&split_bytes[i]);
        }
        // Decrypt each file
        for i in 0..split_bytes.len() {
            let new_filename = file.get_filename_without_extension();
            let file_to_decrypt = file::File::new(&format!("{}.gpg", i));
            let decrypted_file = file_to_decrypt.decrypt(&passphrase);
            if decrypted_file.exists() {
                let decrypted_bytes = decrypted_file.get_bytes();
                let result_file = file::File::new(&new_filename);
                result_file.create_with_bytes(&decrypted_bytes);
            }
        }
        // In the end, delete each file
        for i in 0..split_bytes.len() {
            std::fs::remove_file(format!("{}.gpg", i)).expect("Deleting the file...");
            if Path::new(&format!("{}", i)).exists() {
                std::fs::remove_file(format!("{}", i)).expect("Deleting the file...");
            }
        }
    }

    println!("You're welcome 🥱");
}

fn get_processed_file(file: &file::File, mode: &Mode) -> file::File {
    if matches!(mode, Mode::Decrypt) {
        if file.is_png() {
            let hex_file = file.qr_png_to_hex();
            let hexa_file = hex_file.hex_to_hexa();
            hex_file.delete();  
            return hexa_file;
        }
    }
    file.clone()
}

fn find_indexes(bytes: &Vec<u8>, separator_bytes: &Vec<u8>) -> Vec<usize> {
    let mut indexes: Vec<usize> = Vec::new();
    for i in 0..bytes.len() - separator_bytes.len() + 1 {
        let chunk = &bytes[i..i + separator_bytes.len()];
        if chunk == separator_bytes {
            indexes.push(i);
        }

    }
    indexes
}

fn partition_by_separator(bytes: &Vec<u8>, separator_bytes: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut split_bytes: Vec<Vec<u8>> = Vec::new();
    let indexes = find_indexes(&bytes, &separator_bytes);
    for i in 0..indexes.len() {
        if i == 0 {
            split_bytes.push(bytes[0..indexes[i]].to_vec());
        } else {
            split_bytes
                .push(bytes[indexes[i - 1] + separator_bytes.len()..indexes[i]].to_vec());
        }
    }
    split_bytes
}