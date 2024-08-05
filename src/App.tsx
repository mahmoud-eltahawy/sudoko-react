import { useEffect, useState } from 'react'
import { BoardContext, BoardContextSetter } from './BoardContext';
import { Board } from './Board';
import {is_valid_sudoku,level} from 'blazing-fast'
import { WinModal } from './WinModal';

export const SIZE = 9; 

const success = "bg-lime-950"
const failure = "bg-red-950"

function App() {
  const [index , set_index] = useState(0);
  const [board,set_board] = useState(level(0))
  const [win_modal,set_win_modal] = useState(false);

  useEffect(() => {
    set_board(level(index));
  },[index])

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

  useEffect(() => {
    if (game_ended()) {
      set_win_modal(true)
    }
  },[board])

  is_valid = is_valid_sudoku(board) ? success : failure;

  const on_next_click=() => {
      set_index(x => x + 1);
      set_win_modal(false)               
   };

  return (
      <BoardContext.Provider value={board}>
        <BoardContextSetter.Provider value={set_board}>
        <main className={'h-screen w-screen flex flex-rows-1 gap-5 place-content-center p-5 ' + is_valid}>
          <Board/>
          {
           win_modal 
           ? <WinModal 
             on_next_click={on_next_click}
             /> 
           : <></>
          }
        </main>
        </BoardContextSetter.Provider>
      </BoardContext.Provider>
  )
}

export default App
