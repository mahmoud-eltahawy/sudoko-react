import { SIZE } from "./App";
import { Row } from "./Row";

export function Board() {

    return (
        <div className="grid grid-cols-9 gap-2 text-3xl">
            {tableRows()}
        </div>
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
