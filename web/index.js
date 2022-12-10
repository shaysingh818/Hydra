const { BoardState, add } = require('../pkg/hydra.js');


const board = BoardState.new(3, 3);
console.log(board); 

const result = add(3, 4); 
console.log(result); 
