import { SIZE } from "./App";
import { Column } from "./Column";

export function Board() {
    return (
        <div className="grid grid-cols-9 gap-2 text-3xl">
            {all_columns()}
        </div>
    );
}

function all_columns() {
    const result : JSX.Element[] = new Array(SIZE);
    for (let i = 0; i < (SIZE * SIZE); i += 1) {
        result.push(<Column index={i}/>);
    }
    return result;
}
