# KrustyBeaver
This is a windows keylogger built in Rust that is made to evade antivirus. It uses threads and attempts to look like a simple threads
project. However in one of the threads a keylogger is running. I decided to make this project in Rust to learn more about the language as well as a few other reasons. One of those reasons is that the package manager Cargo is a really great package manager. Rustc is also a great compiler for fast runtimes while having very helpful documentation built in to the compiler. 

# Installation
To install simple use cargo build --release




# Dependencies
This package uses a few rust standard libraries such as threads, networking, and processes. There are 
also some external crates that are imported such as clokwerk which is a rust scheduling package similar to crontab. There
is also the winapi crate that is used for interfacing windows.

# Anti Virus Detection
Note that this may be detected still by virus total as the signature are used more and analyzed. However, it is still functional in sandbox evasion and 
antivirus evasion. The main sandbox detection is a simple cpu check that can be configured easily. The main feature that makes it harder for static analysis
is how Rust compiles their binaries to statically compile making the binary a lot bigger than necessary and harder to reverse engineer. 
There are also a few other things to avoid antivirus detection, one being variable renaming and renaming imports. This is one method that makes it harder to statically analyze binaries.
