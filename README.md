<div align="center">

<img src="https://user-images.githubusercontent.com/124709666/218282007-9eef7a92-dff2-44ea-afeb-9a67a3b3f2c3.jpg" width="200" height="200" />

<div id="user-content-toc">
  <ul>
    <summary><h1 style="display: inline-block;">HexaGuard</h1></summary>
  </ul>
</div>

Multi-passphrase multi-content in a single encrypted file.

</div>

## ⚠️ Warning

This tool is just a proof of concept, almost a joke, I made it only to satisfy my curiosity and learn more about the Rust programming language.

Using this will result in **serious data loss**. See "problems" on "how does it work" to understand more (I did this in two days heh).

If you are really worried about protecting crypto assets, nmemonics in your hardware wallet are a superset of this.

## 🌱 Demo

https://user-images.githubusercontent.com/124709666/218742345-1732334b-cf25-4f68-814d-681558aca72e.mp4c

## Requirements

- GPG (to actually encrypt/decrypt)
- xxd (to transform to hexadecimal)

Optionals: 
- qrencode (to generate qr) (`apt install qrencode`)
- zbar (to decode qr) (`apt install zbar-tools`)
- convert (to get PDF from qr)

## 🔧 How does it work?

![image](https://user-images.githubusercontent.com/124709666/218283119-188016c3-b65b-41bb-ab35-2be97e742819.png)

Each file gets encrypted on its own using the `gpg` command, then a separator string of bytes is added at the end, and appended to the `.hexa` file.

When decrypting, the program parses the binary to partition it by this separator string of bytes, then writes each chunk to a file (restoring the encrypted file). Once all encrypted files are created, it tried to decrypt them one by one, as soon as it finds one it can decrypt, then that's the one it generates.

### Problems:

This list is a vastly incomplete list of problems:

1. The separator string could appear as part of the output of the encryption algorithm, this would effectively break the program.
2. There's no support for using the same passphrase for multiple files, one would get "lost".
3. Encrypting with this program is only lost-access-to-the-code away from forever losing whatever was encrypted.
4. To the eyes of someone who knows, it would be trivial that there are actually two encrypted files stored next to each other.
