# CRLF-Converter
Convert CRLF files to LF and vice versa.

# Usage
Download the project using `git clone https://github.com/frengor/CRLF-Converter.git`  
Build it with `cargo build --release`  
Enjoy!  

```
$ crlf-converter --help
CRLF-Converter 0.1.1
fren_gor <goro@frengor.com>
Convert CRLF files to LF and vice versa

USAGE:
    crlf-converter [FLAGS] <file-to-convert>...

FLAGS:
        --crlf-to-lf    Every CRLF in the file(s) will be converted to LF. This is the default option
    -h, --help          Prints help information
        --lf-to-crlf    Every LF in the file(s) will be converted to CRLF
    -V, --version       Prints version information

ARGS:
    <file-to-convert>...    The file(s) to convert
```
