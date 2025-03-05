# How Build
`cargo run build --release`

# How to run
1. copy the binary at `target/release/game_of_life_evolution.exe` to the root project
2. run `game_of_life_evolution.exe <iterations> <split_amount> <sequence_pattern_to_split>`
example 
```
game_of_life_evolution.exe 100000 9 011000000100100000110000000111010000000000100110000111011100110001111100000011000
game_of_life_evolution.exe 100000 12 000001000000000010100111000000010001000010011101011000101110100100000000011000000000000000101010000001010001110100000001110100001001000000000111
```

# What the binary do?
the binary will calculate the hashrate of specific bitlife sequence and put the result inside `result/<sequence_pattern>.json`
