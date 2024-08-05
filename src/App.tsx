import {  useState} from 'react'
import { BoardContext, BoardContextSetter } from './BoardContext';
import { Board } from './Board';
import {is_valid_sudoku} from 'blazing-fast'

export const SIZE = 9; 

const success = "bg-lime-950"
const failure= "bg-red-950"


function App() {
  const [board,set_board] = useState(new Uint8Array(SIZE * SIZE).fill(0))

  let is_valid = success; 

  const game_ended = () => {
    if (!is_valid) {
      return false;
    }

    for (const x of board) {
      if (!x) {
        return false;
      }
    }

    return true;
  }

  try {
    is_valid = is_valid_sudoku(board) ? success : failure;
  } catch(_e) {}

  return (
      <BoardContext.Provider value={board}>
        <BoardContextSetter.Provider value={set_board}>
        <main className={'h-screen w-screen flex flex-rows-1 gap-5 place-content-center p-5 ' + is_valid}>
          <Board/>
          {
              game_ended() ? "you win" : ""
          }
        </main>
        </BoardContextSetter.Provider>
      </BoardContext.Provider>
  )
}

export default App
