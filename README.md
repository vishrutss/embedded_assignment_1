# Life
## Submitted by: Vishrut Sharma

For this assignment I have implemented [Conway's game of life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) for the MB2 board. First I followed the steps to setup the board as taught in class and implemented blinky to see if the code was working. I then created 2 functions randomize and complement for this program.

The randomize function takes a mutable reference to a Pcg64 random number generator and a mutable reference to the image grid. At first I had some trouble writing the code for this function as I had not used nanorand before. I then referred the class repository [poll-button](https://github.com/pdx-cs-rust-embedded/poll-button/blob/main/src/main.rs) which helped me figure out how to use the nanorand library. The randomize function iterates over each cell in the grid and assigns it a random value of either 0 or 1 using the generate_range method of the random number generator.

The complement function takes a mutable reference to the image grid and iterates over each cell in the grid. It then inverts the value of each cell. I implemented a simple if else condition to do this. If the value of the cell is 0, it is changed to 1 and vice versa.

Before the main loop I called the randomize function to randomize the board. Then I implemented the above mentioned functions in the main loop. First I called the life function in life.rs to update the board according to life rules. Then I wrote an if condition to check if the board is empty. If it is empty, the delay function to delay for 0.5 seconds is called. Then if a button is not pressed the randomize function is called. I then created an if else condition to check whether button A or button B was pressed.

If button A is pressed the randomize function is called. If button B is pressed the complement function is called and a delay of 0.5 seconds is applied so that button presses are ignored. If no button is pressed normal life steps are taken.

I did notice one thing. At the start when I call the randomize function before the main loop it always generates the same board pattern. But when I press button A it generates a new random board pattern. I am not sure why this happens. I tried to debug it but could not figure it out.

I referred the following repositories to help me with this assignment:

https://github.com/pdx-cs-rust-embedded/poll-button/blob/main/src/main.rs

https://github.com/pdx-cs-rust-embedded/mb2-leds/blob/main/src/main.rs
