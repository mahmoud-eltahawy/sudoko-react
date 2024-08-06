import { createContext } from "react";


export const BoardContext = createContext<Uint8Array | undefined>(undefined);
export const TriggerContext = createContext<number>(0);
export const BoardContextSetter = createContext<undefined | React.Dispatch<React.SetStateAction<Uint8Array>>>(undefined);
