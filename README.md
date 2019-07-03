# Codename: Dune
An action 2D platformer made with the [Amethyst](https://amethyst.rs/) game engine.

This is my *firstish* attempt at a game (I did write a basic snake game some time back). Through this project, I aim to get better at coding in [Rust](https://www.rust-lang.org/), get familiar with the Amethyst game engine and start learning game development in general. I also hope that this project will help other Amethyst and game dev beginners like me in some way or the other.

Currently, it is a :warning:**WORK IN PROGRESS**:warning: and there is still a lot to be done before it reaches a playable state. Also, the code is far from perfect and there is a lot of scope for improvement. I will keep enhancing the code incrementally, as I go along. 

## Running the game
```
# Clone the repo
git clone https://github.com/krankur/code-name-dune.git
cd code-name-dune

# Run
cargo run —release 
```

## Features:
- [x] Basic map using Tiled
- [x] Animation using prefabs (Main character run, jump, shoot, etc.)
- [x] Basic 2D physics (gravity, velocity, collision detection, etc.)
- [x] Lazy spawning of entities
- [ ] One complete level (enemies, full map, etc.)
- [ ] Parallax
- [ ] Start, Pause and Game Over screens
- [ ] Game mechanics / rules (points, lives, etc.)
- [ ] Audio
- [ ] [nphysics](https://nphysics.org/) integration (using [specs-physics](https://github.com/amethyst/specs-physics/))
- [ ] Documentation
- [ ] Tests

## Credits / Thanks:
- The awesome Amethyst community for helping me out whenever I got stuck. Special thanks to Ben, doomy, JoshMcguigan, Alve, azriel, Dispersia, Moxinilian, torkleyy, Jojolepro, kel, jaynus, Frizi.
- [ansimuz](https://ansimuz.itch.io/) for all the cool assets used in this game.
