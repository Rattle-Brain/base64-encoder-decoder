# Base 64 Encoder and decoder
This project is a Base64 encoder and decoder written in Rust. It's current functionalities are encode a plaintext string and decode a Base64 string, however, it only takes ASCII characters.

A future update will include a way to specify the charset wanted to encode and decode, so that the program knows what to use.

Later on I will include other functionalities for the program to allow to encode and decode other kinds of files (TXT, DOCX, PDF) and dump the Base64 clump to a .txt file. To recover the original file, just decode the file and the original file will be created.

At least that's the idea.
## DISCLAIMER
This program has been created as a way for me to practice Rust. It's not meant to be used in the real world, since it's not tested and secured. Bugs may appear and files can be lost.

Feel free to fork, improve and play with it.

# USAGE
### Encode
You can execute the program by running in your terminal
```
cargo build --release
base64-encoder-decoder.exe -e "String to encode"
```
This will encode the string "String to encode" and print the result to the standard output.

### Decode
For this, you can run the following commands
```
cargo build --release
base64-encoder-decoder.exe -d "U3RyaW5nIHRvIGVuY29kZQ=="
```
This will decode the Base64 string to regular plain text.

### Other Commands
```
base64-encoder-decoder.exe --help
base64-encoder-decoder.exe --version
```
Self explanatory. Shows the version of the program and the help in case you need it.

### Note
The command ```cargo build --release``` has to be run just once.
