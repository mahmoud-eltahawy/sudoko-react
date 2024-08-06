import React from "react";
import { SIZE } from "./App";
import { Column } from "./Column";


export const Board = React.memo(BoardInner);

function BoardInner() {
    return (
        <div className="grid grid-cols-9 gap-2 text-5xl">
            {all_columns()}
        </div>
    );
}

function all_columns() {
    const result : JSX.Element[] = new Array(SIZE);
    for (let i = 0; i < (SIZE * SIZE); i += 1) {
        result.push(<Column key={i} index={i}/>);
    }
    return result;
}
