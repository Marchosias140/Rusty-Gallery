# Rusty-Gallery
Wallpaper gallery web app written in Rust.
Features a randomizer and several tags. If you are going to change the tags, make sure to change them both in the Rust code and in the HTML code, which hadles the buttons on the web interface.
The app by default uses the localhost (127.0.0.1), but you can change it to make it available for other devices. It uses the port 3000.
There is a Podman version, which I have in another of my repos but I will put it here to make things more tidy.





Installation

``` cargo check``` 
to see if it works
Then



``` cargo run``` 

to try the binary, or




``` cargo build --release``` 


to compile the fully optimized binary.




The binary will be in target/release

The binary must be in the same folder as the static folder, so it can find the wallpapers.


Enjoy!
