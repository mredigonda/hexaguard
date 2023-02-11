use inquire::Select;
use inquire::Text;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::Command;
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

    let file = file::File::new(&base_filename);
    if file.filename.ends_with(".png") {
        file.qr_png_to_hex();
        println!("Converted to hex");
        return;
    }

    let filename = get_processed_filename(&base_filename, &mode);

    println!("Filename: {}", filename);

    assert!(!matches!(&mode, Mode::Decrypt) || filename.ends_with(".hexa"));

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

    // Vector of separator bytes, completely random, based on: "6b 05 8e d2 3f 67 c7 b3 a3 71 a4 12 e1 a6 fa 35"
    let mut separator_bytes = vec![
        0x6b, 0x05, 0x8e, 0xd2, 0x3f, 0x67, 0xc7, 0xb3, 0xa3, 0x71, 0xa4, 0x12, 0xe1, 0xa6, 0xfa,
        0x35,
    ];
    let separator_bytes_size = separator_bytes.len();

    if matches!(mode, Mode::Encrypt) {
        encrypt(&filename, &passphrase);

        let hexa_filename = Text::new("Where?").prompt().expect("No result filename.");
        // Check if hexa_filename ends in ".hexa"
        if !hexa_filename.ends_with(".hexa") {
            println!("Result filename must end in .hexa");
            return;
        }

        let mut existing_bytes = get_file_bytes(&hexa_filename);
        let encrypted_filename = filename + ".gpg";
        let mut new_bytes = get_file_bytes(&encrypted_filename);
        let mut concatenated_bytes = Vec::new();
        concatenated_bytes.append(&mut existing_bytes);
        concatenated_bytes.append(&mut new_bytes);
        concatenated_bytes.append(&mut separator_bytes);
        // Write to file, overwriting the existing file
        File::create(&hexa_filename)
            .expect("Creating the result file...")
            .write_all(&concatenated_bytes)
            .expect("Writing to the result file...");

        let hex_filename = hexa_filename.replace(".hexa", ".hex");

        // Then, convert the file to hexadecimal with command xxd
        Command::new("xxd")
            .arg("-p")
            .arg(&hexa_filename)
            // And then save the output to a file
            .arg(&hex_filename)
            .output()
            .expect("xxd failed to start");

        // Now let's use this .hex file to get the QR code
        // First, we need to convert the .hex file into a .png file
        let png_filename = hex_filename.replace(".hex", ".png");
        Command::new("qrencode")
            .arg("-o")
            .arg(&png_filename)
            .arg("-s")
            .arg("10")
            .arg("-l")
            .arg("H")
            .arg("-m")
            .arg("1")
            .arg("-d")
            .arg("300")
            .arg("-r")
            .arg(&hex_filename)
            .output()
            .expect("qrencode failed to start");
        // Explanation of everything:
        // -o: output file
        // -s: size of the QR code
        // -l: error correction level, H is the highest
        // -m: margin
        // -d: DPI
        // -r: read from file

        // Now let's create a PDF file with the QR code
        let pdf_filename = hex_filename.replace(".hex", ".pdf");
        // The PDF should fit in a A4 page, we print the QR code
        // and then below can be whitespace
        Command::new("convert")
            .arg(&png_filename)
            .arg(&pdf_filename)
            .output()
            .expect("convert failed to start");
        // Explanation of everything:
        // -size: size of the image
        // -units: units of the size
        // -density: DPI
        // -background: background color
        // -alpha: background color
        // -gravity: where to put the image
        // -extent: size of the image
        // -append: append the image to the previous one
        // The last argument is the input file

        // Command::new("convert")
        //     .arg(&png_filename)
        //     .arg(&pdf_filename)
        //     .output()
        //     .expect("convert failed to start");

        // Instead, we will write the bytes but as text, in hexadecimal
        // let mut result_file = File::create(&hexa_filename).expect("Creating the result file...");
        // for byte in &concatenated_bytes {
        //     result_file.write_all(format!("{:02x}", byte).as_bytes()).expect("Writing to the result file...");
        // }
        // And then we read it and parse it back to binary
        // let mut result_file = File::open(&hexa_filename).expect("Opening the result file...");
        // let mut result_file_bytes = Vec::new();
        // result_file.read_to_end(&mut result_file_bytes).expect("Reading the result file...");
        // let mut result_file_hexa = String::new();
        // for byte in &result_file_bytes {
        //     result_file_hexa.push_str(&format!("{:02x}", byte));
        // }
        // let mut result_file_bytes = Vec::new();
        // for i in 0..result_file_hexa.len() {
        //     if i % 2 == 0 {
        //         let byte = u8::from_str_radix(&result_file_hexa[i..i + 2], 16).expect("Parsing the byte...");
        //         result_file_bytes.push(byte);
        //     }
        // }
        // result_file.write_all(&result_file_bytes).expect("Writing to the result file...");

        // After the process, delete the encrypted file
        std::fs::remove_file(encrypted_filename).expect("Deleting the encrypted file...");
        // And also the .hex file
        std::fs::remove_file(hex_filename).expect("Deleting the .hex file...");
    } else {
        // Decrypt
        println!("File to decrypt: {}", filename);

        let hexa_bytes = get_file_bytes(&filename);

        // Save all indexes where we have separator bytes
        // Print hexa bytes, in hex:
        for byte in &hexa_bytes {
            print!("{:02x}", byte);
        }

        let mut indexes: Vec<usize> = Vec::new();
        for i in 0..hexa_bytes.len() {
            // Check if the sequence of separator bytes appears
            if i + separator_bytes_size - 1 < hexa_bytes.len() {
                let mut is_separator = true;
                for j in 0..separator_bytes_size {
                    if hexa_bytes[i + j] != separator_bytes[j] {
                        is_separator = false;
                        break;
                    }
                }
                if is_separator {
                    indexes.push(i);
                }
            }
        }
        // Print indexes
        println!("Indexes: {:?}", indexes);
        // Then get all the bytes between the indexes
        let mut split_bytes: Vec<Vec<u8>> = Vec::new();
        for i in 0..indexes.len() {
            if i == 0 {
                split_bytes.push(hexa_bytes[0..indexes[i]].to_vec());
            } else {
                split_bytes
                    .push(hexa_bytes[indexes[i - 1] + separator_bytes_size..indexes[i]].to_vec());
            }
        }

        println!("Number of files: {}", split_bytes.len());
        for i in 0..split_bytes.len() {
            println!("Size of file {}: {}", i, split_bytes[i].len());
        }

        // Write each to its own file
        for i in 0..split_bytes.len() {
            let mut file = File::create(format!("{}.gpg", i)).expect("Creating the file...");
            file.write_all(&split_bytes[i])
                .expect("Writing to the file...");
        }
        // Decrypt each file
        for i in 0..split_bytes.len() {
            if decrypt(&format!("{}.gpg", i), &passphrase) {
                // If decryption worked, write the decrypted file to the result file
                let decrypted_bytes = get_file_bytes(&format!("{}", i));
                let decrypted_filename = filename.replace(".hexa", "");
                let mut result_file =
                    File::create(&decrypted_filename).expect("Creating the result file...");
                result_file
                    .write_all(&decrypted_bytes)
                    .expect("Writing to the result file...");
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

fn get_processed_filename(filename: &String, mode: &Mode) -> String {
    let mut processed_filename = filename.clone();
    if matches!(mode, Mode::Decrypt) {
        if processed_filename.contains(".png") {
            // Then it's a QR code. We need to convert it to a .hexa file
            // First, to hexadecimal
            let hex_filename = processed_filename.replace(".png", ".hex");

            // This is the command we will run: zbarimg --raw -q <filename> > <filename>
            let output = Command::new("zbarimg")
                .arg("--raw")
                .arg("-q")
                .arg(&filename)
                // We pipe the output to a file, to write the file
                // Like if we were doing "> <filename>"
                .stdout(std::process::Stdio::piped())
                .arg(&hex_filename)
                .output()
                .expect("zbarimg failed to start");

            // Write the stdout of the above to a file:
            let mut file = File::create(&hex_filename).expect("Creating the file...");
            file.write_all(&output.stdout)
                .expect("Writing to the file...");

            // println!(
            //     "Output: {}",
            //     String::from_utf8(output.stdout).expect("Error converting output to string")
            // );

            processed_filename = processed_filename.replace(".png", ".hexa");
            // Then convert from hexadecimal to binary
            // This is the command we will run: xxd -r -p <filename> <filename>
            Command::new("xxd")
                .arg("-r")
                .arg("-p")
                .arg(&hex_filename)
                .stdout(std::process::Stdio::piped())
                .arg(&processed_filename)
                .output()
                .expect("xxd failed to start");
            println!(
                "QR code converted to .hexa file called {}",
                processed_filename
            );
        } else if !processed_filename.contains(".hexa") {
            panic!("The file must be a .png or a .hexa file")
        }
    }
    processed_filename
}
