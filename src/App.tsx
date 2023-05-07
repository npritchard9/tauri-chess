import { createEffect, createSignal, For, onMount } from "solid-js";
import { createStore, produce } from "solid-js/store";
import { invoke } from "@tauri-apps/api/tauri";
import { Board } from "../src-tauri/bindings/Board";
import { Piece } from "../src-tauri/bindings/Piece";
import { Color } from "../src-tauri/bindings/Color";

type ChessBoard = ChessPiece[][];
type ChessPiece = {
	bg: string;
} & Piece;

interface Coord {
	r: number;
	f: number;
}

async function make_move() {
	let new_board: Board = await invoke("make_move", { moves: moves(), from: from(), to: to() });
	let new_chess_board: ChessBoard = new_board.squares as ChessBoard;
	for (let rank of new_board.squares) {
		for (let file of rank) {
			let bg = "bg-slate-800";
			if ((file.rank + file.file) % 2 === 0) bg = "bg-slate-600";
			new_chess_board[file.rank][file.file] = { bg: bg, ...file };
		}
	}
	setBoard(_prev => new_chess_board);
	setTurn(new_board.turn);
	setLastSquare(to());
	setTo();
	setFrom();
	toggleLast(lastSquare()?.r!, lastSquare()?.f!);
}

async function get_moves() {
	setMoves(await invoke("get_moves", { from: currentSquare() }));
	for (let r = 0; r < board.length; r++) {
		for (let f = 0; f < board[r].length; f++) {
			for (let move of moves()) {
				if (move[0] === r && move[1] === f) {
					toggleAvail(r, f);
				}
			}
		}
	}
}

const [board, setBoard] = createStore<ChessBoard>([]);

const toggleAvail = (r: number, f: number) => {
	setBoard(
		produce(board =>
			board.map(row =>
				row.map(file =>
					file.rank === r && file.file === f
						? (file.bg = "bg-orange-100")
						: (file.bg = file.bg)
				)
			)
		)
	);
};

const toggleCurrent = (r: number, f: number) => {
	setBoard(
		produce(board =>
			board.map(row =>
				row.map(file =>
					file.rank === r && file.file === f
						? (file.bg = "bg-orange-500")
						: (file.bg = getNewColor(file.rank, file.file))
				)
			)
		)
	);
};

const toggleLast = (r: number, f: number) => {
	setBoard(
		produce(board =>
			board.map(row =>
				row.map(file =>
					file.rank === r && file.file === f
						? (file.bg = "bg-orange-300")
						: (file.bg = getNewColor(file.rank, file.file))
				)
			)
		)
	);
};

const [turn, setTurn] = createSignal<number>(0);
const [moves, setMoves] = createSignal<number[][]>([[]]);
const [from, setFrom] = createSignal<Coord>();
const [to, setTo] = createSignal<Coord>();
const [currentSquare, setCurrentSquare] = createSignal<Coord>();
const [lastSquare, setLastSquare] = createSignal<Coord>();

const getNewColor = (r: number, f: number) => {
	if (board[r][f].bg === "bg-orange-500" || board[r][f].bg === "bg-orange-100") {
		return (r + f) % 2 === 0 ? "bg-slate-600" : "bg-slate-800";
	}
	return board[r][f].bg;
};

function App() {
	async function set_initial_board() {
		let new_board: Board = await invoke("get_board");
		let new_chess_board: ChessBoard = new_board.squares as ChessBoard;
		for (let rank of new_board.squares) {
			for (let file of rank) {
				let bg = "bg-slate-800";
				if ((file.rank + file.file) % 2 === 0) bg = "bg-slate-600";
				new_chess_board[file.rank][file.file] = { bg: bg, ...file };
			}
		}
		setBoard(_prev => new_chess_board);
		setTurn(new_board.turn);
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

	createEffect(() => {
		console.log("moves()", moves());
	});

	return (
		<div class="flex flex-col items-center justify-center h-screen">
			<div class="bg-slate-800 rounded-xl p-4 mb-4">
				{turn() % 2 === 0 ? "White" : "Black"} To Play
			</div>
			<div class="grid grid-cols-8 grid-rows-8 border">
				<For each={board}>
					{rank => <For each={rank}>{file => <BoardSquare {...file} />}</For>}
				</For>
			</div>
		</div>
	);
}

const isCurrentSquare = (r: number, f: number) => {
	return r === currentSquare()?.r && f === currentSquare()?.f;
};

const isLastSquare = (r: number, f: number) => {
	return r === lastSquare()?.r && f === lastSquare()?.f;
};

const isCurrentColor = (c: Color) => {
	return c === (turn() % 2 === 0 ? "White" : "Black");
};

const isAvailMove = (r: number, f: number) => {
	for (let move of moves()) {
		if (move[0] === r && move[1] === f) {
			return true;
		}
	}
	return false;
};

function BoardSquare(props: ChessPiece) {
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
	return (
		<div
			onclick={() => {
				if (isCurrentColor(props.color)) {
					setCurrentSquare({ r: props.rank, f: props.file });
					toggleCurrent(props.rank, props.file);
					get_moves();
				}
				if (from() === undefined && isCurrentColor(props.color)) {
					setFrom({ r: props.rank, f: props.file });
				}
				if (from() !== undefined && to() === undefined) {
					if (
						!(props.rank === from()?.r && props.file === from()?.f) &&
						props.color !== board[from()?.r!][from()?.f!].color &&
						isAvailMove(props.rank, props.file)
					) {
						setTo({ r: props.rank, f: props.file });
						make_move();
					}
				}
			}}
			class={`flex items-center justify-center h-20 w-20 text-6xl select-none ${
				isCurrentSquare(props.rank, props.file) && "bg-orange-500"
			}
				${isLastSquare(props.rank, props.file) && "bg-orange-300"}
				${props.bg}
			} ${props.color === "Empty" ? "" : "cursor-pointer"} ${
				props.color === "White" ? "text-white" : "text-black"
			}
			`}
		>
			{piece}
		</div>
	);
}

export default App;
