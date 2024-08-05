import { useEffect, useState } from 'react'
import { BoardContext, BoardContextSetter } from './BoardContext';
import { Board } from './Board';
import {is_valid_sudoku,level} from 'blazing-fast'

export const SIZE = 9; 

const success = "bg-lime-950"
const failure= "bg-red-950"

function App() {
  const [board,set_board] = useState(level(0))
  const [win_modal,set_win_modal] = useState(false);

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
      console.log('game ended')
    }
    
  },[board])

  is_valid = is_valid_sudoku(board) ? success : failure;

  return (
      <BoardContext.Provider value={board}>
        <BoardContextSetter.Provider value={set_board}>
        <main className={'h-screen w-screen flex flex-rows-1 gap-5 place-content-center p-5 ' + is_valid}>
          <Board/>
          {
           win_modal ? <WinModal close={() => set_win_modal(false)}/> : <></>
          }
        </main>
        </BoardContextSetter.Provider>
      </BoardContext.Provider>
  )
}

function WinModal({close} : {close : Function}) {
  const on_click = () => {
    console.log("clicked")
    close();
  };
  
  return (
    <div 
      role='dialog'
      className='bg-white grid grid-cols-1 size-96 text-center text-4xl rounded-lg border-4 absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2'>
      <h2 className='m-5 p-5'>you win 🏆</h2>
      <button 
        onClick={on_click}
        className='border-2 rounded-full'
      >next level</button>
    </div>
  )
}

export default App
