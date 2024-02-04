# Life
## Submitted by: Vishrut Sharma

For this assignment I have implemented the game of life for the MB2 board. First I followed the steps to setup the board as taught in class and implemented blinky to see if the code was working. I then created 2 functions randomize and complement for this program.

The randomize function takes a mutable reference to a Pcg64 random number generator and a mutable reference to the image. At first I had some trouble writing the code for this function as I had not used nanorand before. I then referred the class repository [poll-button](https://github.com/pdx-cs-rust-embedded/poll-button/blob/main/src/main.rs) which helped me figure out how to use the nanorand library. The randomize function iterates over each pixel in the image and assigns it a random value of either 0 or 1 using the generate_range method of the random number generator.

The complement function takes a mutable reference to the image and iterates over each pixel in the image. It then inverts the value of each pixel. I implemented a simple if else condition to do this. If the value of the pixel is 0, it is changed to 1 and vice versa.

Before the main loop I called the randomize function to randomize the board. Then I implemented the above mentioned functions in the main loop. First I called the life function in life.rs to update the board. Then I wrote an if condition to check if the board was empty. If it was empty, I called the delay function to delay for 0.5 seconds. Then if a button was not pressed the randomize function is called. I then created an if else condition to check whether button A or button B was pressed.

If button A was pressed the randomize function is called. If button B was pressed the complement function is called and a delay of 0.5 seconds is applied so that button presses are ignored. If no button is pressed normal life steps are taken.

I referred the following repositories to help me with this assignment:

https://github.com/pdx-cs-rust-embedded/poll-button/blob/main/src/main.rs

https://github.com/pdx-cs-rust-embedded/mb2-leds/blob/main/src/main.rs
