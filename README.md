# rust-webserver

Project repo for ITCS 4102: Programming Languages

Web server demo in the Rust programming language

You will need to install Cargo to make get this program to run properly
Cargo is Rust's build system and package manager. Most Rustaceans use cargo to manage Rust

On Linux and macOS systems:
"curl https://sh.rustup.rs -sSf | sh"
After download and installation you should see this apear if you are successful:
"Rust is installed now. Great!"

On Windows got to this link and install the .exe file, then follow the instructions:
https://doc.rust-lang.org/cargo/getting-started/installation.html



If the above steps executed with out error run the command "cargo run".
The server will now be running on your computer. 
The default configuration is to bind to all ip addresses and run the server on the standard HTTP port 80 this can me changed by editing the .env file.

Now that the server is running you can open your web browser and enter http://127.0.0.1:7878/ this will request the default demonstration page in the sever.
The WWW directory in the server folder is where web pages are stored, any html or .png files you put in the www folder can be requested by the prowser. 
For example http://127.0.0.1:7878/test.png

Your clients interaction can be seen in the server console.
