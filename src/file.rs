use std::process::Command;
use std::fs::File as FSFile;
use std::io::Write;

pub struct File {
    pub filename: String,
}

impl File {
    pub fn new(filename: &String) -> File {
        File { filename: filename.clone() }
    }

    pub fn qr_png_to_hex(&self) -> File {
        if self.filename.ends_with(".png") {
            let hex_filename = self.filename.replace(".png", ".hex");
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

    pub fn delete(&self) {
        std::fs::remove_file(&self.filename).expect("Deleting the file...");
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
