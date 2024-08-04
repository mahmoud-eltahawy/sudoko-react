import { useState} from 'react'
import { Row } from './Row';
import { BoardContext, BoardContextSetter } from './BoardContext';
// import {is_valid_sudoku} from 'blazing_fast'

function App() {
  const [rows,set_rows] = useState(new Uint8Array(81).fill(0))
  return (
    <div className='flex flex-rows-1 place-content-center p-5'>
      <BoardContext.Provider value={rows}>
        <BoardContextSetter.Provider value={set_rows}>
          <Table/>
        </BoardContextSetter.Provider>
      </BoardContext.Provider>
    </div>
  )
}

function Table() {

  return (
    <table className="table-fixed text-3xl">
      <tbody>
        {mapArray()}
      </tbody>
    </table>
  )
}

function mapArray() {
  const result = [];
  for (let row = 0; row < (9 * 9); row += 9) {
    result.push(<Row 
          key={row} 
          index={row}
          />
        )
  }
  return result
}

export default App
