import { ChangeEvent, useContext, useMemo } from "react";
import { valid_chars, ValidChar } from "./shared";
import { BoardContext, BoardContextSetter, TriggerContext } from "./BoardContext";
import { which_section } from "blazing-fast";

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
            className={`size-20 text-center rounded-lg ${which_color(index)} ${!disabled ? "bg-yellow-100" : ""}` }
            pattern="^[0-9]$"
            value={value}
            disabled={disabled}
            onInput={on_input}
         />
    );
}

function which_color(index : number) :string {
    switch (which_section(index)) {
        case 0:
           return "text-red-500"         
        case 1:
           return "text-amber-500"         
        case 2:
           return "text-blue-500"         
        case 3:
           return "text-cyan-500"         
        case 4:
           return "text-violet-500"         
        case 5:
           return "text-fuchsia-500"         
        case 6:
           return "text-lime-500"         
        case 7:
           return "text-rose-500"         
        case 8:
           return "text-yellow-500"         
        default:
           return ""
    };
}
