# breakout: MB2 Rust Breakout Game
Bart Massey 2023

This is a demo of
[Breakout](https://en.wikipedia.org/wiki/Breakout_%28video_game%29)
on the [MicroBit v2](https://microbit.org/new-microbit/)
(MB2). Due to the limited display and controls, Breakout is
a challenge on the MB2: this code is a preliminary
demonstration of what might be possible.

## Build and Run

To use this program, you will need to have a potentiometer
(pot) connected to the edge connector of the MB2 to drive
the paddle. The [Adafruit Dragon
Tail](https://www.adafruit.com/product/3695), a breadboard,
and a 100K PCB through-hole pot are one way to get started.
Connect pin 1 of the pot to +3.3V, pin 3 to ground, and pin
2 to P0 (Ring 0) on the MB2 edge connector.

The program can be built with `cargo build --release`. It
can be uploaded with `cargo embed` via `cargo embed
--release`, `probe-run` via `cargo run --release`, using any
CMSIS-DAP connector, or via the MB2 virtual SD card.

# License

This work is licensed under the "MIT License". Please see the file
`LICENSE.txt` in this distribution for license terms.

# Breakout Writeup
### Tyler Taormina - taormina@pdx.edu

- For this assignment we were given three major changes to implement in the Breakout game code. We needed to add audio feedback for any collision in the game, create dynamic brightness for the blocks to indicate how many times they have been hit by the ball, and we needed to add functionality for the buttons that are built into the microbit. I started my work on this project by first working on the sound. Removing the game start up beeps was straightforward, as was adding 3 beeps to indicate the end of a game. Handling the collisions simply required adding 'beep' calls in the appropriate places of the 'step' function in game.rs. I added two new beeps named 'short\_beep' and 'long\_beep' which aims to differentiate between events as they occur in the game. It doesn't work as well as it could but it's a start. I think as I get more time to work on this, I will attempt to have the beeps output different tones which will make differentiating them much easier. After I finished the sounds up, I moved onto dimming the blocks after they had been hit once. This is handled at the bottom of the 'step' function where we loop through our blocks array and modify the 'raster' values accordingly. I added another 'if' and changed around some of the logic so it now checks for values (1, 2) and sets raster to values (5, 9). 9 being full brightness for a block and 5 being about half brightness. This works well and adds a nice visual element to the game. Lastly, I worked on adding button functionality. To do this I created a new file called button.rs which contains a 'Button' struct and implementations for creating, and reading from the microbit built-in buttons. I used the knob.rs file to help me build the 'read' function with the main difference being that we need to store a value when using the buttons. This value needs to get increased and decreased based on the button being pressed, and then this value is used to either move the paddle left or right. As for allowing both the button and knob to be used in gameplay, I added an 'if/else' block in game.rs where we attempt to unwrap the value from the buttons first. If 'None' is found then unwrapping the knob is attempted. If 'None' is found again, then we enter attract mode. This works well and so far has given me a good gameplay experience. Other than that I made little changes to game.rs, mainly around when a ball is lost and when it hits the paddle. I made it slightly more forgiving and I think it does play a little better but I'll let other players be the judge of that. Overall, this was a really cool project that I learned a lot from. I hope to keep writing embedded software throughout the rest of my education and career. Thanks so much. 

Tyler Taormina 
