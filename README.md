<div align="center">

![hexaguard_logo](https://user-images.githubusercontent.com/124709666/218282007-9eef7a92-dff2-44ea-afeb-9a67a3b3f2c3.jpg)

</div>

# HexaGuard

Multi-password multi-content in a single encrypted file.
 
## Warning

Do not use this project seriously. No guarantees are provided. It is just a toy proof of concept, and using it could result in serious data loss.

## Demo

## TODO

- [ ] Use a real separator string of bytes that is **guaranteed** not to appear in the encrypted output of GPG
- [ ] Generate a fancier PDF, in A4 format, that can go unnoticed

## Requirements

- GPG (to actually encrypt/decrypt)
- xxd (to transform to hexadecimal)
- qrencode (to generate qr)
- zbar (to decode qr)
  - Use command: `zbarimg --raw <qr_file_name>`
- convert (to get PDF from qr)

## Encrypt

- Run: `hexaguard`
- Choose: "encrypt"
- Write filename to encrypt (called `filename`)
- Write passphrase twice
- Write `.hexa` file destination (if it doesn't exist, it will create it)
- If succesful:
  - The `.hexa` file is modified
  - The file: `filename.png` will contain an image of the QR code
  - The file: `filename.pdf` will contain a PDF with the QR code

## Decrypt

- Run: `hexaguard`
- Choose: "decrypt"
- Write filename to encrypt (called `filename`)
  - It can be a `.hexa` file, or
  - It can be a `.png` file with the QR code
- Write passphrase twice
- If successful, you will get a `filename` without extensions with the raw data