# HexaGuard

Multi-password multi-content in a single encrypted file.

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