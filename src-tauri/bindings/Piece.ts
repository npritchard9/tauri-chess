// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Color } from "./Color";
import type { ControlledBy } from "./ControlledBy";
import type { PieceName } from "./PieceName";

export interface Piece { name: PieceName, color: Color, rank: number, file: number, controlled_by: ControlledBy, }