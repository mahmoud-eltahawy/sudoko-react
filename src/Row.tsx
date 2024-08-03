import { Column } from "./Column";
import {ValidChar} from './shared'

export type RowProps = {
  y : number,
  rows: ValidChar[][],
  set_rows : React.Dispatch<React.SetStateAction<ValidChar[][]>>
};

export function Row(
    { rows, y, set_rows }: RowProps) {

    return (
        <tr>{rows[y].map((_, x) => <Column
            key={x}
            y={y}
            x={x}
            rows={rows}
            set_rows={set_rows} />)}</tr>
    );
}
