# Rusty-Gallery
Wallpaper gallery web app written in Rust.

Updates:

- Added a samples folder with configurations inspired in two waifus from the anime that changed my life.


About the code





Overall this app is made as a template that can be filled with any kind of theme. 



Features a randomizer and several example tags. If you are going to change the tags, make sure to change them both in the Rust code and in the HTML code, which handles the buttons on the web interface.







The randomizer also displays a quote, you can change it or add more to make it more random.






The app by default uses the localhost (127.0.0.1), but you can change it to make it available for other devices. It uses the port 3000.







There is a Podman version for easier deployment and control, which currently only has the Dockerfile, but the docker-compose.yml will be available at some point of the current timeline. 














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


Podman Instructions



Build the image, the binary must be in the same folder as the Dockerfile so it can be copied into the image, it's much easier than compiling the whole thing again.

``` sudo podman build gallery .``` 




Run it with 


``` sudo podman run -t --rm --net=host -v /your/wallpapers/folder:/home/static/wallpapers:Z gallery``` 


Enjoy!
