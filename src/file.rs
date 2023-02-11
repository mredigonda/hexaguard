use std::fs::File as FSFile;
use std::io::Write;
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

    pub fn delete(&self) {
        std::fs::remove_file(&self.filename).expect("Deleting the file...");
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

    pub fn get_filename_with_extension(&self, new_extension: &str) -> String {
        // Remove characters from the end until we find a ".", then add ".hex"
        let mut new_filename = self.filename.clone();
        while !new_filename.ends_with(".") {
            new_filename.pop();
        }
        new_filename.push_str(new_extension);
        new_filename
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
