import { Row } from "./Row";

export function Board() {

    return (
        <table className="table-fixed text-3xl">
            <tbody>
                {mapArray()}
            </tbody>
        </table>
    );
}

function mapArray() {
    const result = [];
    for (let row = 0; row < (9 * 9); row += 9) {
        result.push(<Row
            key={row}
            index={row} />
        );
    }
    return result;
}
