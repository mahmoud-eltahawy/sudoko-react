import { useEffect,  useState } from 'react'
import { BoardContext, BoardContextSetter, TriggerContext } from './BoardContext';
import { Board } from './Board';
import {is_valid_sudoku,is_sudoku_board_full,get_level} from 'blazing-fast'
import { WinModal } from './WinModal';

export const SIZE = 9; 

const success = "bg-lime-950"
const failure = "bg-red-950"


function App() {
  const [level , set_level] = useState(0);
  const [trigger , set_trigger] = useState(0);
  const [board,set_board] = useState(get_level(0))
  const [win_modal,set_win_modal] = useState(false);

  const is_valid = is_valid_sudoku(board);
  const is_valid_style = is_valid ? success : failure;

  useEffect(() => {
    set_board(get_level(level))
    set_trigger(x => x + 1)
  },[level])

  useEffect(() => {
    if (is_valid && is_sudoku_board_full(board)) {
      set_win_modal(true)
    }
  },[board])

  const on_next_click=() => {
      set_level(x => x + 1);
      set_win_modal(false)               
  };


  return (
      <BoardContext.Provider value={board}>
        <BoardContextSetter.Provider value={set_board}>
          <TriggerContext.Provider value={trigger}>
        <main className={'h-screen w-screen flex flex-wrap gap-5 place-content-center p-5 ' + is_valid_style}>
          <h2 className='w-screen text-center text-3xl text-white'>{`LEVEL ${level + 1}`}</h2>
          <Board/>
          <WinModal on_next_click={on_next_click} show={win_modal}/> 
        </main>
          </TriggerContext.Provider>
        </BoardContextSetter.Provider>
      </BoardContext.Provider>
  )
}

export default App
