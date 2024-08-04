import { createContext } from "react";


export const BoardContext = createContext<Uint8Array | undefined>(undefined);
export const BoardContextSetter = createContext<undefined | React.Dispatch<React.SetStateAction<Uint8Array>>>(undefined);
