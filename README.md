# Life
Ashton Sawyer

This is a program to play 
[Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
on the 5x5 LED grid on the Micro:bit-v2. 

It runs at 10 frames per second. It starts with a random board and takes steps
for the game of life each frame unless:
-  While button A is held, the board is re-randomized every frame. 
- Otherwise, when button B is pressed and not ignored, the board is inverted. 
Then button B is ignored for 5 frames.
- If all the cells are off, the program waits 5 frames before starting with a
new random board (barring any buttons being pressed) 

## Development Process
Writing this actually went better than I was expecting. My limited experience
with Rust worried me, but once I started it went relatively quickly. I have
to assume that it's not the most idiomatic code, but it passes clippy. 

### Imports
The piece that took the most effort to get compiling was the `Cargo.toml` and 
`use` statements. Cargo is foreign to me so I was very unsure of everything I 
typed, making the process *much* slower than necessary. I stole `.cargo` from
`blinky-rs` and generally found it useful to keep an eye on the Zulip. 

### randomize\_board()
Since the code for the actual game of life was provided, the next most basic
functionality seemed like the ability to randomize the board. This meant
reading the documentation for both `nanorand` (to actually generate the 
values) and the board's RNG (to seed `nanorand`). 

`nanorand` was a little confusing to use because `Pcg64::new()` didn't work 
the way I expected would. Switching to `new_seed()` was easy enough. I was 
expecting the hardware RNG to be more difficult, but I found it to be very
straighforward. 

One thing that still confuses me is the `SeedableRng` that's imported in the
example. It's my understanding that you want to seed once on init and then let
it be, meaning `new_seed` makes it redundant. I'm not sure if I'm 
misunderstanding the library or how seeding works in general. Either way, I'm
content with my solution for now.

### In Polling We Trust
I decided to start working with the buttons without interrupts because it
seemed simpler. Generally I'm happy with this decision. I think the code is 
clean and easy to follow, but I do think the timing takes a bit of a hit in
exchange. I'm okay with this because 

1. It's updating every 100 ms

It's updating quickly enough that it can be difficult to fully follow by eye.
If there's a few milisecond lag between the button being pressed and the leds
responding, oh well. 

2. Perfect responsiveness isn't critical for the use case

If occasionally a button is pressed and released such that the board doesn't 
register it, it's not difficult as a user to just press the button again. This
mostly applies to button B, as button A is likely to be held down longer. 

3. There isn't much of a reason to push both buttons at the same time

Button B is only supposed to be registered when button A isn't being pressed. 
Also, it doesn't make sense to have two button presses register within a single
frame. That would be skipping a step visually and just isn't necessary.

### Delays
There are two delays to consider: waiting after death, and ignoring after
pressing button B. Waiting was a straightforward since the highest it
will ever be is 6 and it's only checked if the board is dead. Ignoring had a 
little more to consider. 

Ostensibly, `ignore` could be a u8. This would make sense in many ways because
it also only needs to count up to 5 or 6 and would save space on the limited
storage that an embedded system provides. On the otherhand, a u8 that increases
every frame could realistically overflow between presses to button B. To handle
this, I could have checked every frame whether `ignore` was over 6 and capped
it there, but that was ugly and seemed like unnecessary processing. That, along
with the fact that this is a small, simple program led me to leaving `ignore`
as a larger int. 

It's worth noting that it is still possible for `ignore` to overflow. However,
because it would overflow at such a large number I don't think it would under
normal use. And if it did, it would be at max half of a second before the user
could use button B, and so wouldn't be much of a bother if they had to press it
a second time. 

### General Rustiness
If I'm being entirely honest, I don't know why certain variables needed to be
mutable. With the LEDs it makes sense. With the RNG? Less so. I often set the
variables as unmutable only to be scolded by the compiler. The same was true of
passing by reference. 

I could go read and figure out why it needed to be done that way. I probably
will in the near future. Until then, though, it was very nice to not have to 
worry too much about the details because I could trust the compiler to catch
those issues for me and tell me accurately where it was and how to fix it. 
