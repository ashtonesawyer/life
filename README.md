# Life
Ashton Sawyer

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
content with my solution in this moment. 
