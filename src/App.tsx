import {  useState} from 'react'
import { BoardContext, BoardContextSetter } from './BoardContext';
import { Board } from './Board';
import {is_valid_sudoku} from 'blazing-fast'

export const SIZE = 9; 

const success = "bg-lime-950"
const failure= "bg-red-950"

function App() {
  const [rows,set_rows] = useState(new Uint8Array(SIZE * SIZE).fill(0))

  let is_it = success; 
  try {
    is_it = is_valid_sudoku(rows!) ? success : failure;
  } catch(_e) {}

  return (
      <BoardContext.Provider value={rows}>
        <BoardContextSetter.Provider value={set_rows}>
        <main className={'h-screen w-screen flex flex-rows-1 gap-5 place-content-center p-5 ' + is_it}>
          <Board/>
        </main>
        </BoardContextSetter.Provider>
      </BoardContext.Provider>
  )
}

export default App
