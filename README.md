# GravRogue

A simple gravity based roguelike game that uses Newtonian physics to govern the battlefield

## What it is planned to be like

GravRogue will (hopefully) feature a wide variety of enemy types, fun weapons that use gravity as their primary factor, and lots of planets to explore. 

## What it currently has

The current build is a very simple demo of the gravity system and player movement. Every object (right now) is equally affected by gravity exerted by the other masses around them. Another thing the demo contains is multithreading for physics. This project was more of a medium for my obsession with multithreaded anything that I can multithread. The multithreading system works by calculating gravitational forces in one pass using non-mutatable data, then applying the acceleration from the forces in the second pass. Is this useful right now? To be honest, nope! The current game is way too small for parallelism to even matter, in fact, the game runs better singlethreaded as there is less overhead. But eventually, as the game grows, the multithreading will definately increase performance.

## What can you do in the demo

Not too much at the moment, mostly just fly around and try to get into orbits. The game is quite small scale with the planets typically having a radius of less than 1.5km. Feel free to test movement in gravitational environments, it would be quite interesting to move around, as you can't stop in space, unless you want to fall into the sun or the planet below. The planetary orbits should be *stable* as in if it was an isolated system, it only the planet and the sun, it should hypothetically be able to orbit forever (I hate float precision errors). Also, you get a thrust-to-mass ratio of 10 to visit different planets more easily. Each planet as a different density value, so bigger planets aren't necessarily heavier, Earth is densest planet in the solar system, but it isn't the largest. The same thing applies to stars. 

## What can you expect the next build to have?

The next build will focus on moons and kinetic energy dumb projectiles. These projectiles will perform differently based on the gravitational environment that you are firing on. Most likely, in the full game, weapons will be adjusted based on the planet they were generated on, so expect to find better weapons on heavier planets.

## CONTROLS

**W, A, S, D** to accelerate, acceleration is dependent on the surface gravity of the planet you spawn on multiplied by 10.

**Q, E** to zoom in and out.

**R** to exit to main menu.

**Esc** to exit the game.
