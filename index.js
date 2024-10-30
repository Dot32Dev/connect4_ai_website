import init, { State } from "./pkg/connect4_ai_website.js";
// init().then(() => {
//   greet("WebAssembly");
// });

init().then(() => {
  const rows = 6;
  const columns = 7;

  let turn = "red";

  let game = new State();

  function initialise_board() {
    const board = document.querySelector(".board");

    for (let col = 1; col <= columns; col++) {
      const column = document.createElement("div");
      column.classList.add("column");
      column.setAttribute("data-col", col);

      column.addEventListener("click", (e) => {
        let target_cell;
        for (const cell of column.children) {
          if (!cell.hasChildNodes()) {
            target_cell = cell;
            break;
          }
        }

        if (target_cell) {
          const row = target_cell.getAttribute("data-row");
          const piece = document.createElement("div");
          piece.classList.add(turn);
          piece.classList.add("piece");
          piece.style.transform = `translateY(${-(50 * (rows + 0 - row) + 5)}px)`;
          target_cell.appendChild(piece);
          setTimeout(() => {
            piece.style.transform = "translateY(0px)";
          }, 10);

          // greet(`${turn} placed a piece in row ${col}`);
          game.place(col);

          const turn_text = document.querySelector(`span.${turn}`);
          turn_text.classList.remove(turn);
          turn = turn === "red" ? "blue" : "red";
          turn_text.classList.add(turn);
          turn_text.innerHTML = `${turn[0].toUpperCase() + turn.slice(1)}'s`;
        }
      });

      for (let row = 1; row <= rows; row++) {
        const cell = document.createElement("div");
        cell.classList.add("cell");
        cell.setAttribute("data-row", row);

        column.appendChild(cell); // Add cell to column
      }

      board.appendChild(column); // Add column to board
    }
  }

  initialise_board();
});
