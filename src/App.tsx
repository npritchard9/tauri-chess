import { createEffect, createSignal, For, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import { Board } from "../src-tauri/bindings/Board";
import { Piece } from "../src-tauri/bindings/Piece";

interface Coord {
	r: number;
	f: number;
}

async function make_move() {
	let new_board: Board = await invoke("make_move", { from: from(), to: to() });
	setBoard(new_board);
	setLastSquare(to());
	setTo({ r: -2, f: -2 });
	setFrom({ r: -1, f: -1 });
}

const [board, setBoard] = createSignal<Board>({ squares: [], turn: 0 });
const [from, setFrom] = createSignal<Coord>({ r: -1, f: -1 });
const [to, setTo] = createSignal<Coord>({ r: -2, f: -2 });
const [currentSquare, setCurrentSquare] = createSignal<Coord>({ r: -1, f: -1 });
const [lastSquare, setLastSquare] = createSignal<Coord>({ r: -2, f: -2 });

function App() {
	async function set_initial_board() {
		setBoard(await invoke("get_board"));
	}

	onMount(() => {
		set_initial_board();
	});

	createEffect(() => {
		console.log("from()", from());
		console.log("to()", to());
		console.log("currentSquare()", currentSquare());
		console.log("lastSquare()", lastSquare());
	});

	return (
		<div class="flex items-center justify-center h-screen">
			<div class="grid grid-cols-8 grid-rows-8 border">
				<For each={board()?.squares}>
					{rank => <For each={rank}>{file => <BoardSquare {...file} />}</For>}
				</For>
			</div>
			<div>{board().turn % 2 === 0 ? "White" : "Black"}</div>
		</div>
	);
}

const isCurrentSquare = (r: number, f: number) => {
	return r === currentSquare().r && f === currentSquare().f;
};

const isLastSquare = (r: number, f: number) => {
	return r === lastSquare().r && f === lastSquare().f;
};

function BoardSquare(props: Piece) {
	let bg_color = "bg-slate-800";
	let piece = "";
	switch (props.name) {
		case "King":
			piece = "♚";
			break;
		case "Queen":
			piece = "♛";
			break;
		case "Rook":
			piece = "♜";
			break;
		case "Bishop":
			piece = "♝";
			break;
		case "Knight":
			piece = "♞";
			break;
		case "Pawn":
			piece = "♟";
			break;
	}
	if ((props.rank + props.file) % 2 === 0) bg_color = "bg-slate-600";
	return (
		<div
			onclick={() => {
				setCurrentSquare({ r: props.rank, f: props.file });
				if (from().r === -1 && props.color !== "Empty") {
					setFrom({ r: props.rank, f: props.file });
				}
				if (
					from().r !== -1 &&
					to().r === -2 &&
					!(props.rank === from().r && props.file === from().f) &&
					props.color !== board().squares[from().r][from().f].color
				) {
					setTo({ r: props.rank, f: props.file });
					make_move();
				}
				// if (props.color === "Empty") {
				// 	setCurrentSquare({ r: -1, f: -1 });
				// 	if (from().r !== -1 && to().r !== -2) {
				// 		setFrom({ r: -1, f: -1 });
				// 		setTo({ r: -2, f: -2 });
				// 	}
				// } else {
				// 	setCurrentSquare({ r: props.rank, f: props.file });
				// }
				// if (from().r === -1) {
				// 	setFrom({ r: props.rank, f: props.file });
				// } else if (to().r === -2) {
				// 	setTo({ r: props.rank, f: props.file });
				// 	make_move();
				// }
			}}
			class={`flex items-center justify-center h-20 w-20 text-6xl ${
				isCurrentSquare(props.rank, props.file) ? "bg-orange-500" : bg_color
			}
				${isLastSquare(props.rank, props.file) ? "bg-orange-300" : bg_color}
			} ${props.color === "Empty" ? "" : "cursor-pointer"} ${props.color === "White" && "text-white"} ${
				props.color === "Black" && "text-black"
			}`}
		>
			{piece}
		</div>
	);
}

export default App;
