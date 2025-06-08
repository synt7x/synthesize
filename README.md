# Synthesize

Learn audio synthesis interactively with a simple introductory crash course. Teaches the basics of oscillators and filters through interactive learning.

## Building

You must have Rust and CMake installed to build this project. Once those prequisites are installed, simply run `cargo build --release` to create a build of the executable.

## Inspiration

I was inspired to work on this project due to my fascination with audio synthesis, and wanted to try doing audio synthesis at a low level myself. This was my first time writing a proper audio engine, and I hoped to gain new insight into the inner workings of audio formats and pipelines. I also hoped to learn the ropes of the `sdl3` Rust API by writing a custom rendering engine. Through this project, I hope that I can spread my passion of synthesizers to an audience that may be interested but daunted by the complexity.

## How we built it

Built with a custom rendering and audio engine, this project only depends on a single library, SDL3, and does everything from scratch (even font rendering!) I create my own bitmap font for rendering, as well as write a component tree system for rendering complex interactions with buttons, a waveform viewer, sliders, box sizing, and visual styling. I then integrated an `Educator` element that provides a tutorial for the user, guiding them through their experience in an effective and educational manner.

## Challenges I ran into

Keeping the state and propagating events to elements is a major hurdle in a memory strict language such as Rust. Lots of behind the scene tricks had to occur to properly wrangle UI elements and get them rendering in an efficient manner. The borrow checker also made managing textures (specifically, regarding text rendering) extremely difficult, as they only live for the lifetime of the `TextureCreator`, meaning they must have their lifetimes explicitly extended in order to be passed to the elements they are rendered by.

## Accomplishments that I am proud of

Custom behind the scenes audio synthesis: I basically created a digital synthesizer from scratch that is used internally to generate all the audio in the application. Overcoming the borrow checker proved difficult especially with so many dynamic objects in the form of UI elements, however, the dynamic nature of the elements allowed rapid iteration once the system was established.

## What we learned

Dynamic structs inheriting from traits in Rust, building UI/rendering from scratch, audio generation and timing from scratch, and how to ergonomically manage dynamic types in Rust. I tried to challenge myself by creating the systems entirely on my own, with no external libraries or dependencies (besides the base library sdl3) to create something that is uniquely mine.

## What's next for Synthesize

Adding more features! One feature I would like to add is Attack, Decay, and Release, a key component of many synthesizers, changing the way individual notes sound. Now that the groundwork is laid out through the element system, further rapid iteration can now occur. I also want to expand the tutorial to expand the learning aspect of the application.