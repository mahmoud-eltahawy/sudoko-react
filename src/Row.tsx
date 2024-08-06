import { SIZE } from "./App";
import { Column } from "./Column";


export function Row(
    { index }: { index : number, }) {

    const map = () => {
        const result : JSX.Element[]= new Array(SIZE);
        for (let x = index; x < (index + SIZE);x++) {
            result.push(<Column
                key={x}
                index={x}/>)
        }
        return result
    }

    return map();
}
