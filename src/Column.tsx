import { ChangeEvent } from "react";
import { valid_chars, ValidChar } from "./shared";
import { RowProps } from "./Row";



export function Column({ rows, index, set_rows }: RowProps) {
    const on_input = (e: ChangeEvent<HTMLInputElement>) => {
        const new_value = +e.target.value;
        if (valid_chars.includes(new_value)) {
            set_rows(xs => {
                const new_xs  = new Uint8Array(xs);
                new_xs[index] = new_value as ValidChar;
                return new_xs;
            });
        }
    };

    return (
        <td className="size-24 border-2">
            <input
                className="size-24 text-center"
                type='number'
                value={rows[index]}
                onInput={on_input}
             />
        </td>
    );
}
