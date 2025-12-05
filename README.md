Disclaimer: This project uses anime titles and character names (e.g., from Love Live!) 
only as placeholders for demonstration purposes. 
All characters, names, and related intellectual property belong to their respective owners. 
This project is not affiliated with, endorsed by, or sponsored by the official rights holders. 
The source code is licensed under the MIT License.


# Rusty-Gallery
Wallpaper gallery web app written in Rust and HTML, all tidy and redeable within the ``` main.rs```  file. What you modify match in both codes, becaue one handles what the app will grab and the other will handle how is it presented in the web interface, more or less.

Updates:

- Added a samples folder with configurations inspired in two waifus from the anime that changed my life.
- Added docker-compose.yml file for easy management.
- Made the network configuration more precise.


About the code





Overall this app is made as a template that can be filled with any kind of theme. 



Features a randomizer and several example tags. The tags index the images according to the name of the tag being present within the name of the file (i.e if the tag is 'waifu' and the file is 'waifu1.jpg', it will be indexed in the 'waifu' tag).







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

``` sudo podman build rusty-gallery .``` 


Then



```sudo podman compose up -d ``` 


Enjoy!
