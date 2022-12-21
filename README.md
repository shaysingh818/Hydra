# Summary
Hydra is a system for implementing reinforcement learning algorithms. This paradigm of learning requires a representation of an agent, state and environment. Games, specifically with two dimensional environments, can easily represent RL problems. Algorithms can be used to manage the state and create learning strategies. The purpose of this project, is to learn about various methods to represent a game environment, create libraries for implementing reinforcement learning algorithms and understand the fundamentals of creating “simple” puzzles that agents can solve.  


## Compiling Rust to Web Assembly
```
wasm-pack build --target nodejs
```


## Install NPM Modules
```
npm install
```

## Run Server
```
npm start
```

## Sync Database Schema
Refer to ```create_user.sql``` for creating DB users
```
mysql -u hydra_admin -p < db.sql
```

## Testing the Rust Library
```
cargo test
```


## Expected Unit Test Results
```
running 25 tests
test board::board_instance::test_agent_turn_cycle ... ok
test board::board_instance::test_board_availability ... ok
test board::board_instance::test_init_board ... ok
test board::board_instance::test_add_agent_to_board ... ok
test board::board_instance::test_make_move ... ok
test board::board_instance::test_place_piece ... ok
test board::board_instance::test_resize_board ... ok
test board::board_instance::test_pop_piece ... ok
test engine::minimax::tictactoe::test_minimax::board_eval::test_negmax_eval ... ok
test engine::minimax::tictactoe::test_minimax::board_eval::test_static_evaluation ... ok
test game::test_c4::game_logic::test_horizontals ... ok
test game::test_c4::game_logic::test_left_right_diagonal ... ok
test game::test_c4::game_logic::test_all_horizonals ... ok
test game::test_c4::game_logic::test_all_verticals ... ok
test game::test_c4::game_logic::test_right_left_diagonal ... ok
test game::test_c4::game_logic::test_verticals ... ok
test game::test_tictactoe::game_logic::test_higher_dimension_boards ... ok
test game::test_tictactoe::game_logic::test_horizontals ... ok
test game::test_tictactoe::game_logic::test_left_right_diagonal ... ok
test game::test_tictactoe::game_logic::test_right_left_diagonal ... ok
test game::test_tictactoe::game_logic::test_verticals ... ok
test game::test_tictactoe::wasm_integration::test_set_board ... ok
test engine::minimax::tictactoe::test_minimax::test_minimax::test_ab_negamax ... ok
test engine::minimax::tictactoe::test_minimax::test_minimax::test_negamax ... ok
test engine::minimax::tictactoe::test_minimax::test_minimax::test_minimax ... ok
```

## Want to Contribute
If anyone is interested in this project. The open issues are fair game and can be tackled by anyone. Those are the items I don't want to spend time working on at the moment and wouldn't mind someone else doing it. For any questions, contact: shalinsingh818@gmail.com
