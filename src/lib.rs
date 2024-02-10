


//! # Hydra
//!
//! Hydra is a library for implementing and simulating reinforcement
//! learning algorithms. The library comes with generic structures to 
//! represent game environments. It also comes with pre built environments
//! for popular 2d game board environments.
//!
//! # Design
//! The design of Hydra is split into 4 major components. There are
//! modules for creating multidimensional arrays, abstract environments,
//! and abstract reinforcment learning algorithms. Multidimensional arrays
//! are the foundation of the state structure. The state structure lets you create
//! game states of any dimension. 
//!
//! # Generic Environment Structures
//! * `NDArray` - Multi dimensional array which states are built off of
//! * `State` - Grid state structure used for representing multidimensional environments
//! * `Agent` - Structure that represents Agent/Player that can be added to state environment
//!
//! # Current Game Environments
//! * `Connect4` - Two player game on 6 X 7 board
//! * `TicTacToe` - Two player game on 2d environment
//! * `GridWorld` - Grid environment for simulating markov decision process
pub mod structures;
pub mod environment;  
pub mod games;


