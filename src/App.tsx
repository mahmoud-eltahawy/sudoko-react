import {useState} from 'react'
import { Row } from './Row';
import { ValidChar } from './shared';

function App() {
  return (
    <div className='flex flex-rows-1 place-content-center p-5'>
      <Table/>
    </div>
  )
}

function Table() {
  const [rows,set_rows] = useState<ValidChar[][]>(new Array(9).fill(new Array(9).fill('.')));

  return (
    <table className="table-fixed text-3xl">
      <tbody>
        {
        rows.map((_,row) => <Row 
          key={row} 
          y={row} 
          rows={rows} 
          set_rows={set_rows}/>)
        }
      </tbody>
    </table>
  )
}

export default App
