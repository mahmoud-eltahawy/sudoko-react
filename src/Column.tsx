import { ChangeEvent, useContext, useMemo } from "react";
import { valid_chars, ValidChar } from "./shared";
import { BoardContext, BoardContextSetter, TriggerContext } from "./BoardContext";

export function Column({ index }: { index : number }) {
    const rows = useContext(BoardContext);
    const trigger = useContext(TriggerContext);
    const set_rows = useContext(BoardContextSetter);

    const on_input = (e: ChangeEvent<HTMLInputElement>) => {
        const new_value = +e.target.value;
        if (valid_chars.includes(new_value)) {
            set_rows!(xs => {
                const new_xs  = new Uint8Array(xs);
                new_xs[index] = new_value as ValidChar;
                return new_xs;
            });
        }
    };

    const value = rows![index] || "" 

    const disabled = useMemo(() => rows![index] !== 0,[trigger])

    return (
        <input
            id={`column-number-${index}`}
            className="size-20 text-center rounded-lg"
            pattern="^[0-9]$"
            value={value}
            disabled={disabled}
            onInput={on_input}
         />
    );
}
