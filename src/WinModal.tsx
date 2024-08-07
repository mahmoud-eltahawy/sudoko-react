
type WinModalParams = { 
  on_next_click : () => void
  show : boolean
};

export function WinModal({ on_next_click, show} : WinModalParams) {

    if (!show) {
       return; 
    }

    return (
        <div
            role='dialog'
            className='bg-slate-500 grid grid-cols-1 size-96 text-center rounded-lg absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2'>
            <h2 className="m-5 p-5 text-5xl">
                <span className='bg-gradient-to-r from-blue-400 via-red-400 to-yellow-300 text-transparent bg-clip-text'>you win </span>
                <span>🏆</span>
            </h2>
            <button
                onClick={on_next_click}
                className='border-2 rounded-full text-4xl m-5 hover:text-5xl border-yellow-500 hover:border-blue-500'
            >
                <span className='bg-gradient-to-r from-yellow-400 via-red-400 to-blue-300 text-transparent bg-clip-text'>next level</span>
            </button>
        </div>
    );
}
