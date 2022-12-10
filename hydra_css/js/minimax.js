function openForm() {
	document.getElementById("agent-form-modal").style.display = "block"; 
}

function closeForm() {
	document.getElementById("agent-form-modal").style.display = "none"; 
}

function cell_clicked(rows, cols){
	console.log("Rows: " + rows + " " + "Cols: " + cols);
}

function render_board(rows, cols){

	/* find board element */ 
	let board_cover = document.getElementById("minimax-board");


	/* loop through rows and columns and render */
	for(let i = 0; i < rows; i++){

		/* append board row element */ 
		const board_row = document.createElement("div"); 
		board_row.classList.add("minimax-board-row"); 

		for(let j = 0; j < cols; j++){

			/* create board cell with top and left params */
			const board_cell = document.createElement("div");
			board_cell.classList.add("minimax-board-cell");

			/* add click listener */ 
			board_cell.addEventListener("click", function() {
				cell_clicked(i, j);
            });

			/* create text element for board cell */ 
			const cell_text = document.createElement("div");
			const text_node = document.createTextNode("O"); 
			cell_text.classList.add("minimax-board-cell-text");
			cell_text.append(text_node); 
			board_cell.append(cell_text);
			board_row.append(board_cell); 
			
		}
	
		/* add to row */ 
		board_cover.append(board_row); 

	}
}

render_board(3,3); 
