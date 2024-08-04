import { createContext } from "react";


export const BoardContext = createContext(new Uint8Array(81).fill(0));
export const BoardContextSetter = createContext<undefined | React.Dispatch<React.SetStateAction<Uint8Array>>>(undefined);
