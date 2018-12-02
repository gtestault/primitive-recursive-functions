

[Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) at type level in Rust.

```
Output for BLINKER type: 
 
   Generation 1:
   - - - - - - - - - - - 
   - - - - - - - - - - - 
   - - - - - - - - - - - 
   - - - - - - - - - - - 
   - - - - - - - - - - - 
   - - - - X X X - - - - 
   - - - - - - - - - - - 
   - - - - - - - - - - - 
   - - - - - - - - - - - 
   - - - - - - - - - - - 
   - - - - - - - - - - - 
   
   
   
   
   Generation 2:
   - - - - - - - - - - - 
   - - - - - - - - - - - 
   - - - - - - - - - - - 
   - - - - - - - - - - - 
   - - - - - X - - - - - 
   - - - - - X - - - - - 
   - - - - - X - - - - - 
   - - - - - - - - - - - 
   - - - - - - - - - - - 
   - - - - - - - - - - - 
   - - - - - - - - - - - 
   
   
   Generation 3:
   - - - - - - - - - - - 
   - - - - - - - - - - - 
   - - - - - - - - - - - 
   - - - - - - - - - - - 
   - - - - - - - - - - - 
   - - - - X X X - - - - 
   - - - - - - - - - - - 
   - - - - - - - - - - - 
   - - - - - - - - - - - 
   - - - - - - - - - - - 
   - - - - - - - - - - - 
```

compile time for 15 generations on intel-i5 4460: ```10m 01s```