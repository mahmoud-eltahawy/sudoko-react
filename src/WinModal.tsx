
type WinModalParams = { 
  on_next_click : () => void
};

export function WinModal({ on_next_click} : WinModalParams) {

    return (
        <div
            role='dialog'
            className='bg-white grid grid-cols-1 size-96 text-center text-4xl rounded-lg border-4 absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2'>
            <h2 className='m-5 p-5'>you win 🏆</h2>
            <button
                onClick={on_next_click}
                className='border-2 rounded-full'
            >next level</button>
        </div>
    );
}
