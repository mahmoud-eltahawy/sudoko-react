import { useState} from 'react'
import { BoardContext, BoardContextSetter } from './BoardContext';
import { Board } from './Board';
// import {is_valid_sudoku} from 'blazing_fast'

function App() {
  const [rows,set_rows] = useState(new Uint8Array(81).fill(0))
  return (
      <BoardContext.Provider value={rows}>
        <BoardContextSetter.Provider value={set_rows}>
        <div className='flex flex-rows-1 place-content-center p-5'>
          <Board/>
        </div>
        </BoardContextSetter.Provider>
      </BoardContext.Provider>
  )
}

export default App
