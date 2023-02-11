use std::fs::File as FSFile;
use std::io::Write;
use std::io::Read;
use std::process::Command;

#[derive(Clone)]
pub struct File {
    pub filename: String,
}

impl File {
    pub fn new(filename: &String) -> File {
        File {
            filename: filename.clone(),
        }
    }

    pub fn qr_png_to_hex(&self) -> File {
        if self.is_png() {
            let hex_filename = self.get_filename_with_extension("hex");
            // This is the command we will run: zbarimg --raw --quiet <filename> > <filename>
            let output = Command::new("zbarimg")
                .arg("--raw")
                .arg("--quiet")
                .arg(&self.filename)
                .output()
                .expect("zbarimg failed");
            // Write the stdout of the above to a file:
            let mut file = FSFile::create(&hex_filename).expect("Creating the file...");
            file.write_all(&output.stdout)
                .expect("Writing to the file...");
            File::new(&hex_filename)
        } else {
            panic!("File must be a png");
        }
    }

    pub fn hex_to_hexa(&self) -> File {
        if self.is_hex() {
            let hexa_filename = self.get_filename_with_extension("hexa");
            // This is the command we will run: xxd -r -p <filename> <filename>
            Command::new("xxd")
                .arg("-r")
                .arg("-p")
                .arg(&self.filename)
                .stdout(std::process::Stdio::piped())
                .arg(&hexa_filename)
                .output()
                .expect("xxd failed to start");
            File::new(&hexa_filename)
        } else {
            panic!("File must be a hex");
        }
    }

    pub fn hexa_to_hex(&self) -> File {
        if self.is_hexa() {
            let hex_filename = self.get_filename_with_extension("hex");
            // This is the command we will run: xxd -p <filename> <filename>
            Command::new("xxd")
                .arg("-p")
                .arg(&self.filename)
                .stdout(std::process::Stdio::piped())
                .arg(&hex_filename)
                .output()
                .expect("xxd failed to start");
            File::new(&hex_filename)
        } else {
            panic!("File must be a hexa");
        }
    }

    pub fn hex_to_qr_png(&self) -> File {
        if self.is_hex() {
            let png_filename = self.get_filename_with_extension("png");
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
                .arg(&self.filename)
                .output()
                .expect("qrencode failed to start");
            File::new(&png_filename)
        } else {
            panic!("File must be a hex");
        }
    }

    pub fn qr_png_to_pdf(&self) -> File {
        if self.is_png() {
            let pdf_filename = self.get_filename_with_extension("pdf");
            Command::new("convert")
                .arg(&self.filename)
                .arg(&pdf_filename)
                .output()
                .expect("convert failed to start");
            File::new(&pdf_filename)
        } else {
            panic!("File must be a png");
        }
    }

    pub fn delete(&self) {
        std::fs::remove_file(&self.filename).expect("Deleting the file...");
    }

    pub fn create_with_bytes(&self, bytes: &Vec<u8>) {
        FSFile::create(&self.filename)
            .expect("Creating the result file...")
            .write_all(&bytes)
            .expect("Writing to the result file...");
    }

    pub fn is_png(&self) -> bool {
        self.has_extension(".png")
    }

    pub fn is_hex(&self) -> bool {
        self.has_extension(".hex")
    }

    pub fn is_hexa(&self) -> bool {
        self.has_extension(".hexa")
    }

    pub fn has_extension(&self, extension: &str) -> bool {
        self.filename.ends_with(extension)
    }

    pub fn exists(&self) -> bool {
        std::path::Path::new(&self.filename).exists()
    }

    pub fn get_filename_without_extension(&self) -> String {
        if !self.filename.contains(".") {
            return self.filename.clone();
        }
        // Remove characters from the end until we find a "."
        let mut new_filename = self.filename.clone();
        while !new_filename.ends_with(".") {
            new_filename.pop();
        }
        new_filename.pop(); // Remove the "." as well
        new_filename
    }

    // Example: "test.png" -> "test.hex"
    // Input: "test.png", "hex"
    pub fn get_filename_with_extension(&self, new_extension: &str) -> String {
        let mut new_filename = self.filename.clone();
        if new_filename.contains(".") {
            // Remove characters from the end until we find a ".", then add ".hex"
            while !new_filename.ends_with(".") {
                new_filename.pop();
            }
        } else {
            new_filename.push('.');
        }
        new_filename.push_str(new_extension);
        new_filename
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        if self.exists() {
            FSFile::open(&self.filename)
                .expect("Opening the result file...")
                .read_to_end(&mut buf)
                .expect("Reading the result file...");
        }
        buf
    }

    pub fn encrypt(&self, passphrase: &String) -> File {
        // Run the following command: gpg --batch --passphrase "<passphrase>" -c <filename>
        let mut child = Command::new("gpg")
            .arg("--batch")
            .arg("--passphrase")
            .arg(passphrase)
            .arg("-c")
            .arg(&self.filename)
            .spawn()
            .expect("Encrypting the file...");
        println!("(1) Ran encryption command");
        child.wait().expect("Encrypting the file...");
        println!("(2) Wait finished");
        let encrypted_filename = self.get_filename_with_extension("gpg");
        println!("(3) Encrypted filename: {}", encrypted_filename);
        File::new(&encrypted_filename)
        // If failed, the file will not exist
    }

    pub fn decrypt(&self, passphrase: &String) -> File {
        // Run the following command: gpg --passphrase "<passphrase>" <filename>
        let mut child = Command::new("gpg")
            .arg("--batch")
            .arg("--passphrase")
            .arg(passphrase)
            .arg(&self.filename)
            .spawn()
            .expect("Decrypting the file...");
        child.wait().expect("Decrypting the file...");
        let decrypted_filename = self.get_filename_without_extension();
        File::new(&decrypted_filename)
        // If failed, the file will not exist
    }

    // pub fn convert_qr_png_to_binary_hexa(&self) -> Vec<u8> {
    // if self.filename.ends_with(".png") {
    //     let hex_filename = processed_filename.replace(".png", ".hex");
    //     // This is the command we will run: zbarimg --raw -q <filename> > <filename>
    //     let output = Command::new("zbarimg")
    //         .arg("--raw")
    //         .arg("--quiet")
    //         .arg(&self.filename)
    //         .stdout(std::process::Stdio::piped())
    //         .arg(&hex_filename)
    //         .output()
    //         .expect("zbarimg failed to start");
    //     // Write the stdout of the above to a file:
    //     let mut file = File::create(&hex_filename).expect("Creating the file...");
    //     file.write_all(&output.stdout)
    //         .expect("Writing to the file...");
    // } else {
    //     panic!("File must be a png");
    // }
    // }
}
