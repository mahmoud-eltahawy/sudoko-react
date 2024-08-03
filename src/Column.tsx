import { ChangeEvent } from "react";
import { valid_chars, ValidChar } from "./shared";
import { RowProps } from "./Row";


type ColumnProps = {
  x : number,
} & RowProps;

export function Column({ rows, y, x, set_rows }: ColumnProps) {
    const on_input = (e: ChangeEvent<HTMLInputElement>) => {
        const new_value = e.target.value;
        if (valid_chars.includes(new_value)) {
            set_rows(xs => {
                const new_xs = JSON.parse(JSON.stringify(xs));
                new_xs[y][x] = new_value as ValidChar;
                return new_xs;
            });
        }
    };

    return (
        <td className="size-24 border-2">
            <input
                className="size-24 text-center"
                type='number'
                value={rows[y][x]}
                onInput={on_input}
                onFocus={() => {
                    set_rows(xs => {
                        const new_xs = JSON.parse(JSON.stringify(xs));
                        new_xs[y][x] = '.';
                        return new_xs;
                    });
                }} />
        </td>
    );
}
