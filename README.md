# NPF
Neat Picture Format
## Concept
The concept of NPF is to create a simple file in which are multiple multiple colors values for individiual pixels wich are seperated by a s character for the next value, a p character for next pixel or a n charcter for next line. For Example
```Concept
255s051s047p000s000s000\n
000s000s000p036s137s255
```
## Programm
This programm is a simple concept for making a png to a .npf. It uses Image dependeci.

### Installation
You coulld just download the precompiled linux binary, download the project and compile it yourself or download the project and use the install.sh for linux(note: Rust must be installed for this)
### Usage
```bash
npf-conv [input-file] [output-file]
```