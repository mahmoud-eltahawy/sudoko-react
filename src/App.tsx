import {useState} from 'react'
import { Row } from './Row';
// import {is_valid_sudoku} from 'blazing_fast'

function App() {
  return (
    <div className='flex flex-rows-1 place-content-center p-5'>
      <Table/>
    </div>
  )
}

function Table() {
  const [rows,set_rows] = useState(new Uint8Array(81).fill(0))

  return (
    <table className="table-fixed text-3xl">
      <tbody>
        {mapArray(rows,set_rows)}
      </tbody>
    </table>
  )
}

function mapArray(rows : Uint8Array,set_rows : React.Dispatch<React.SetStateAction<Uint8Array>>) {
  const result = [];
  for (let row = 0; row < 81; row += 9) {
    result.push(<Row 
          key={row} 
          index={row} 
          rows={rows} 
          set_rows={set_rows}/>
        )
  }
  return result
}

export default App
