import { SIZE } from "./App";
import { Row } from "./Row";

export function Board() {

    return (
        <table className="table-fixed text-3xl">
            <tbody>
                {tableRows()}
            </tbody>
        </table>
    );
}

function tableRows() {
    const result : JSX.Element[] = new Array(SIZE);
    for (let row = 0; row < (SIZE * SIZE); row += SIZE) {
        result.push(<Row
            key={row}
            index={row} />
        );
    }
    return result;
}
