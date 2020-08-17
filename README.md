Weather_tray-rs
================

It is what the very descriptive name says, A system tray which shows the weather temperature of the selected place using the [Open Weather API](https://openweathermap.org/).

## How to build it yourself and customize it

If you find this program useful to you and in this early development stage you want to compile your own version here's a list of needs:

* Have [Rust programming language and its toolchain installed.](https://rustup.rs/).
* Do a `cargo fetch` to get the dependencies of this project.

And now you can open the project on the code editor, IDE of your choice and start changing stuff. 

## Future plans and Next up. 

* I plan on making this program more concise, contained on it's own executable, none of this creating a file to load back on the  system tray crate. (DONE) 
* Another thing I want to do is to make easy to change the city and the API key so everybody has it's own key. (done for the API key, now read from ENV)
* And Last Figure out how to make this whole program assynchronous or a way to create a delay to make another request to the API.