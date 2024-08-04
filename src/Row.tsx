import { Column } from "./Column";


export function Row(
    { index }: { index : number, }) {

    return (
        <tr>{mapArray(index)}</tr>
    );
}

function mapArray(y : number) {
    const result = [];
    for (let x = y; x < (y + 9);x++) {
        result.push(<Column
            key={x}
            index={x}/>)
    }
    return result
}
