use inquire::Select;
use inquire::Text;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::fs::File;

fn main() {
    let options = vec!["Encrypt", "Decrypt"];

    let ans = Select::new("What?", options)
        .prompt()
        .expect("No option selected.");
    let filename = Text::new("What filename?").prompt().expect("No filename.");

    // Check that if decrypting, the file ends in .hexa
    if ans == "Decrypt" && !filename.ends_with(".hexa") {
        println!("Filename must end in .hexa");
        return;
    }

    if !Path::new(&filename).exists() {
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

    if ans == "Encrypt" {
        encrypt(&filename, &passphrase);

        let result_filename = Text::new("Where?").prompt().expect("No result filename.");
        // Check if result_filename ends in ".hexa"
        if !result_filename.ends_with(".hexa") {
            println!("Result filename must end in .hexa");
            return;
        }

        let mut existing_bytes = get_file_bytes(&result_filename);
        let encrypted_filename = filename + ".gpg";
        let mut new_bytes = get_file_bytes(&encrypted_filename);
        let mut concatenated_bytes = Vec::new();
        concatenated_bytes.append(&mut existing_bytes);
        concatenated_bytes.append(&mut new_bytes);
        concatenated_bytes.append(&mut vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        // Write to file, overwriting the existing file
        File::create(result_filename)
            .expect("Creating the result file...")
            .write_all(&concatenated_bytes)
            .expect("Writing to the result file...");

        // After the process, delete the encrypted file
        std::fs::remove_file(encrypted_filename).expect("Deleting the encrypted file...");
    } else {
        let hexa_bytes = get_file_bytes(&filename);

        // Save all indexes where we have vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        let mut indexes: Vec<usize> = Vec::new();
        for i in 0..hexa_bytes.len() {
            if i + 5 < hexa_bytes.len() {
                if hexa_bytes[i] == 0x00
                    && hexa_bytes[i + 1] == 0x00
                    && hexa_bytes[i + 2] == 0x00
                    && hexa_bytes[i + 3] == 0x00
                    && hexa_bytes[i + 4] == 0x00
                    && hexa_bytes[i + 5] == 0x00
                {
                    indexes.push(i);
                }
            }
        }
        // Then get all the bytes between the indexes
        let mut split_bytes: Vec<Vec<u8>> = Vec::new();
        for i in 0..indexes.len() {
            if i == 0 {
                split_bytes.push(hexa_bytes[0..indexes[i]].to_vec());
            } else {
                split_bytes.push(hexa_bytes[indexes[i - 1] + 6..indexes[i]].to_vec());
            }
        }

        println!("Number of files: {}", split_bytes.len());
        for i in 0..split_bytes.len() {
            println!("Size of file {}: {}", i, split_bytes[i].len());
        }

        // Write each to its own file
        for i in 0..split_bytes.len() {
            let mut file = File::create(format!("{}.gpg", i)).expect("Creating the file...");
            file.write_all(&split_bytes[i]).expect("Writing to the file...");
        }
        // Decrypt each file
        for i in 0..split_bytes.len() {
            if decrypt(&format!("{}.gpg", i), &passphrase) {
                // If decryption worked, write the decrypted file to the result file
                let decrypted_bytes = get_file_bytes(&format!("{}", i));
                let decrypted_filename = filename.replace(".hexa", "");
                let mut result_file = File::create(&decrypted_filename).expect("Creating the result file...");
                result_file.write_all(&decrypted_bytes).expect("Writing to the result file...");
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

    // Save to file

    // Done

    println!("You're welcome");
}

fn encrypt(filename: &String, passphrase: &String) -> bool {
    // Run the following command: gpg --batch --passphrase "<passphrase>" -c <filename>
    let mut child = Command::new("gpg")
        .arg("--batch")
        .arg("--passphrase")
        .arg(passphrase)
        .arg("-c")
        .arg(filename)
        .spawn()
        .expect("Encrypting the file...");
    let exit_status = child.wait().expect("Encrypting the file...");
    return exit_status.success();
}

fn decrypt(filename: &String, passphrase: &String) -> bool {
    // Run the following command: gpg --passphrase "<passphrase>" <filename>
    let mut child = Command::new("gpg")
        .arg("--batch")
        .arg("--passphrase")
        .arg(passphrase)
        .arg(filename)
        .spawn()
        .expect("Decrypting the file...");
    let exit_status = child.wait().expect("Decrypting the file...");
    return exit_status.success();
}

fn get_file_bytes(filename: &String) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    if Path::new(&filename).exists() {
        let bytes_read = File::open(&filename)
            .expect("Opening the result file...")
            .read_to_end(&mut buf)
            .expect("Reading the result file...");
        println!("Bytes read from result filename: {}", bytes_read);
    }
    buf
}
