import { Column } from "./Column";

export type RowProps = {
  index : number,
  rows: Uint8Array,
  set_rows : React.Dispatch<React.SetStateAction<Uint8Array>>
};

export function Row(
    { rows, index, set_rows }: RowProps) {

    return (
        <tr>{mapArray(rows,set_rows,index)}</tr>
    );
}

function mapArray(rows : Uint8Array,set_rows :  React.Dispatch<React.SetStateAction<Uint8Array>>,y : number) {
    const result = [];
    for (let x = y; x < (y + 9);x++) {
        result.push(<Column
            key={x}
            index={x}
            rows={rows}
            set_rows={set_rows} />)
    }
    return result
}
