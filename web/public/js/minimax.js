

const playerTurn = true; 
const gameOver = false;
const rows = 3; 
const cols = 3; 


class TicTacToeBoard {
	
	constructor(set_rows, set_cols) {
		this.rows = set_rows; 
		this.cols = set_cols; 
		this.matrix = [] 

		/* create matrix */
		for(let i = 0; i  < rows; i++){
			let temp = [] 
			for(let j = 0; j < cols; j++){
				temp.push(0); 
			}
			this.matrix.push(temp); 
		}
	}

	rows() {
		return this.rows; 
	}

	cols() {
		return this.cols; 
	}

	printMatrix() {
		for(let i = 0; i  < rows; i++){
			console.log(this.matrix[i]); 
		}
	}

	placePiece(setRow, setCol, piece) {
		let value = this.matrix[setRow][setCol];
		if(value == 0){
			this.matrix[setRow][setCol] = piece; 
		} else {
			alert("Position already taken"); 
		}
	}

	checkDiagonals(piece) {

		let leftRight = true; 
		let rightLeft = true; 
		let rowCount = this.matrix.length - 1; 
		let colCount = 0; 
	
		for(let i = 0; i < rows; i++){

			let leftRightVal = this.matrix[colCount][rowCount]; 
			let rightLeftVal = this.matrix[colCount][colCount];

			if(leftRightVal !== piece) {
				leftRight = false; 
			}

			if(rightLeftVal !== piece){
				rightLeft = false; 
			}

			colCount += 1; 
			rowCount -= 1; 	
		}

		console.log("RIGHT LEFT: " + rightLeft); 
		console.log("LEFT RIGHT: " + leftRight);

		if(rightLeft == true || leftRight == true){
			return true; 
		}

		return false; 
	}


	checkVerticals(piece) {

		let horiz = false; 
		let vert = false; 
		let rowIndex = 0; 
		let colIndex = 0; 
	
		for(let i = 0; i < rows; i++){

			let tempHoriz = true; 
			let tempVert = true; 

			for(let j = 0; j < cols; j++){

				let value = this.matrix[i][j]; 
				if(value != piece){
					tempHoriz = false; 
				}

				let vert_value = this.matrix[colIndex][rowIndex]; 
				if(vert_value != piece){
					tempVert = false; 
				}
				colIndex += 1;
			}

			if(tempHoriz === true || tempVert === true) {

				if(tempHoriz == true) {
					horiz = true; 
				}
	
				if(tempVert == true) {
					vert = true; 
				}
				break; 
			}


			rowIndex += 1; 
			colIndex = 0; 
		}	

		if(horiz == true || vert == true) {
			return true; 
		}
	
		return false; 
	}

	async minimax() {

		let data = {
			rows: 3, 
			cols: 3, 
			board_state: this.matrix
		}; 

		const response = await fetch("http://localhost:8080/minimax", {
			method: "POST",
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(data)
		}); 
	
		return response.json(); 
	}

}


async function cellClicked(board_state, rows, cols){

	// place peice for player
	board_state.placePiece(rows, cols, 2);
	const cell_text_id = "cell-text-" + rows + "-" + cols; 
    const cell_text = document.getElementById(cell_text_id);
	cell_text.innerHTML = 2; 

	console.log("CURRENT BOARD STATE"); 
	board_state.printMatrix();

	let player_diag = board_state.checkDiagonals(2); 
	let player_vert = board_state.checkVerticals(2);

	if(player_diag === true || player_vert === true){
		alert("YOU WIN!");
		alert("Export your board state for prizes!"); 
	} 

	/* place piece for agent */ 
	let result = await board_state.minimax();
	console.log("MOVE" + result); 
	board_state.placePiece(result.row, result.col, 1);

	const agent_text_id = "cell-text-" + result.row + "-" + result.col; 
    const agent_cell_text = document.getElementById(agent_text_id);
	agent_cell_text.innerHTML = 1; 
	board_state.printMatrix();


	let agent_diag = board_state.checkDiagonals(1); 
	let agent_vert = board_state.checkVerticals(1);

	if(agent_diag === true || agent_vert === true){
		console.log("VERT HORIZ: " + agent_vert);
		console.log("DIAG: " + agent_diag); 
		alert("AGENT HAS WON!"); 
	} 
}



function createBoardInterface(board_state){

	/* find board element */
    let board_cover = document.getElementById("minimax-board");

    for(let i = 0; i < board_state.rows; i++){

		/* append board row element */
       	const board_row = document.createElement("div");
        board_row.classList.add("minimax-board-row");
		board_row.id = "board-row-" + i; 

        for(let j = 0; j < board_state.cols; j++){

        	/* create board cell with top and left params */
            const board_cell = document.createElement("div");
            board_cell.classList.add("minimax-board-cell");
			board_cell.id = "board-cell-" + i + "-" + j; 

            /* add click listener */
            board_cell.addEventListener("click", function() {
            	cellClicked(board_state, i, j);
            });


           	/* create text element for board cell if a piece is placed*/
			let board_value = board_state.matrix[i][j]; 
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


function openForm() {
    document.getElementById("agent-form-modal").style.display = "block";
}

function closeForm() {
    document.getElementById("agent-form-modal").style.display = "none";
}


async function main() {

	/* create board object */ 
	let my_board = new TicTacToeBoard(rows, cols); 
	my_board.printMatrix();
	createBoardInterface(my_board); 
}

main(); 




