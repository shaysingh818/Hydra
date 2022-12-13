
/* global vars */ 
const playerTurn = true; 
const gameOver = false;
const rows = 3; 
const cols = 3; 

let board = []; 

function createBoard(rows, cols) {
	for(let i = 0; i  < rows; i++){
		let temp = [] 
		for(let j = 0; j < cols; j++){
			temp.push(0); 
		}
		board.push(temp); 
	}
}


function printBoard(){
	for(let i = 0; i  < rows; i++){
		console.log(board[i]); 
	}
}

function checkDiagonals(piece) {

	let leftRight = true; 
	let rightLeft = true; 
	let rowCount = board.length - 1; 
	let colCount = 0; 
	
	for(let i = 0; i < rows; i++){

		if(board[colCount][rowCount] != piece){
			leftRight = false; 
		}
	
		if(board[colCount][colCount] != piece){
			leftRight = false; 
		}

		colCount += 1; 
		rowCount -= 1; 
	}

	if(leftRight == true || rightLeft == true) {
		return true; 
	}

	return false; 
}


function checkVerticals(piece) {

	let horiz = true; 
	let vert = true; 
	let rowIndex = 0; 
	let colIndex = 0; 
	
	for(let i = 0; i < rows; i++){

		let tempHoriz = true; 
		let tempVert = true; 

		for(let j = 0; j < cols; j++){

			let value = board[i][j]; 
			if(value != piece){
				tempHoriz = false; 
			}

			if(board[colIndex][rowIndex] != piece){
				tempVert = false; 
			}
			colIndex += 1;
		}

		if(tempHoriz == true) {
			horiz = true; 
		}
	
		if(tempVert == true) {
			vert = true; 
		}

		rowIndex += 1; 
		colIndex += 1; 
	}

	if(horiz == true || vert == true) {
		return true; 
	}
	
	return false; 

}

function placePiece(setRow, setCol, piece){
	board[setRow][setCol] = piece; 
}

function openForm() {
    document.getElementById("agent-form-modal").style.display = "block";
}

function closeForm() {
    document.getElementById("agent-form-modal").style.display = "none";
}

async function cellClicked(rows, cols){
	placePiece(rows, cols, 1);

	/*
	let result = await minimax();
	console.log("MOVE" + result); 
	placePiece(result.row, result.col, 2);
	*/ 
	printBoard();
	renderBoard(rows, cols); 
}


function renderBoard(rows, cols){

    let board_cover = document.getElementById("minimax-board");	
    for(let i = 0; i < rows; i++){

		/* get board row */
		const board_row_id = "board-row" + i; 
       	const board_row = document.getElementById(board_row_id);

        for(let j = 0; j < cols; j++){

        	/* get board cell by id */
			const board_cell_id = "board-cell-" + i + "-" + j;
            const board_cell = document.getElementById(board_cell_id);

           	/* Get text element by id and change it to current board val */
			const board_value = board[i][j]; 
			const cell_text_id = "cell-text-" + i + "-" + j; 
            const cell_text = document.getElementById(cell_text_id);
			cell_text.innerHTML = board_value; 
        }

        /* add to row */
        board_cover.append(board_row);
    }
}


function createBoardInterface(rows, cols){

	/* find board element */
    let board_cover = document.getElementById("minimax-board");

    for(let i = 0; i < rows; i++){

		/* append board row element */
       	const board_row = document.createElement("div");
        board_row.classList.add("minimax-board-row");
		board_row.id = "board-row-" + i; 

        for(let j = 0; j < cols; j++){

        	/* create board cell with top and left params */
            const board_cell = document.createElement("div");
            board_cell.classList.add("minimax-board-cell");
			board_cell.id = "board-cell-" + i + "-" + j; 

            /* add click listener */
            board_cell.addEventListener("click", function() {
            	cellClicked(i, j);
            });


           	/* create text element for board cell if a piece is placed*/
			let board_value = board[i][j]; 
            const cell_text = document.createElement("div");
            const text_node = document.createTextNode(board_value);
            cell_text.classList.add("minimax-board-cell-text");
			cell_text.id = "cell-text-" + i + "-" + j; 
            cell_text.append(text_node);
            board_cell.append(cell_text);
            board_row.append(board_cell);
        }

        /* add to row */
        board_cover.append(board_row);
    }
}


async function minimax() {

	let data = {
		rows: 3, 
		cols: 3, 
		board_state: board
	}; 

	const response = await fetch("http://localhost:8080/tic-tac-toe", {
		method: "POST",
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(data)
	}); 
	
	return response.json(); 

}


function setup() {
	createBoard(rows, cols);
}

async function main() {
	setup(); 
	createBoardInterface(rows, cols); 
}

main(); 




