import init, { State } from "./pkg/connect4_ai_website.js";
// init().then(() => {
//   greet("WebAssembly");
// });
//

// Download the shader
// const response = await fetch("/generation_452_ai.ron");
// const AIData = await response.text();

const rows = 6;
const columns = 7;

let game;

window.win = function () {
  window.inactive = true;
  window.won = true;

  // const turn_text = document.querySelector(`span.${turn}`);
  // turn_text.classList.remove(window.turn);
  window.turn = window.turn === "red" ? "blue" : "red";
  // turn_text.classList.add(window.turn);
  console.log(`${window.turn} won`);
  // const turn_text = document.querySelector(`span.${turn}`).parentElement;
  // turn_text.innerHTML = `<span class="${window.turn}">${window.turn[0].toUpperCase() + window.turn.slice(1)}</span> won!`;

  const turnText = document.querySelector("p"); // Select the p element directly
  turnText.innerHTML = `<span class="${window.turn}">${window.turn[0].toUpperCase() + window.turn.slice(1)}</span> won!`;

  // Create and insert reload message
  const reloadMessage = document.createElement("p");
  reloadMessage.innerHTML =
    '<a href="javascript:location.reload()">Play again</a>';
  reloadMessage.style.cursor = "pointer";
  turnText.parentElement.append(reloadMessage);

  document.body.classList.add("inactive");
};

window.ai_loaded_in_rust = function () {
  window.ai_loaded = true;
};

init().then(() => {
  window.turn = "red";
  game = new State();
  window.inactive = false;
  window.won = false;
  window.ai_loaded = false;

  initialise_board();
  loadAI();
});

async function loadAI() {
  try {
    // const response = await fetch("/generation_452_ai.ron");
    const response = await fetch("/generation_1040_ai.json", {
      headers: {
        Accept: "text/plain", // Force text format
      },
    });
    const AIData = await response.text();
    console.log("Loaded AI data:", AIData.substring(0, 100)); // Log first 100 chars
    game.set_ai(AIData);
    console.log("AI loaded successfully");
  } catch (error) {
    console.error("Failed to load AI:", error);
  }
}

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

      if (target_cell && !inactive) {
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

        const turn_text = document.querySelector(`span.${window.turn}`);
        turn_text.classList.remove(turn);
        window.turn = window.turn === "red" ? "blue" : "red";
        turn_text.classList.add(turn);
        turn_text.innerHTML = `${window.turn[0].toUpperCase() + window.turn.slice(1)}'s`;

        game.place(col - 1);

        if (!window.won) {
          window.inactive = true;
          document.body.classList.add("inactive");

          setTimeout(function () {
            ai_turn();
          }, 500);
        }
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

function ai_turn() {
  window.inactive = true;
  document.body.classList.add("inactive");

  let column = game.ai_place();

  column += 1;
  let target_column = document.querySelector(`[data-col="${column}"]`);
  let target_cell;

  for (const cell of target_column.children) {
    if (!cell.hasChildNodes()) {
      target_cell = cell;
      break;
    }
  }

  if (target_cell) {
    const row = target_cell.getAttribute("data-row");
    const piece = document.createElement("div");
    piece.classList.add("blue");
    piece.classList.add("piece");
    piece.style.transform = `translateY(${-(50 * (rows + 0 - row) + 5)}px)`;
    target_cell.appendChild(piece);
    setTimeout(() => {
      piece.style.transform = "translateY(0px)";
    }, 10);

    const turn_text = document.querySelector(`span.${window.turn}`);
    turn_text.classList.remove(turn);
    window.turn = window.turn === "red" ? "blue" : "red";
    turn_text.classList.add(turn);
    turn_text.innerHTML = `${window.turn[0].toUpperCase() + window.turn.slice(1)}'s`;
  }
  if (!window.won) {
    window.inactive = false;
    document.body.classList.remove("inactive");
  }
}
