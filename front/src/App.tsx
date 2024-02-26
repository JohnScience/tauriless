import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import init, { invoke, encode } from "tauriless-js"

function App() {
  const [count, setCount] = useState(0)
  init().then(() => {
    console.log("tauriless-js initialized!");
    const encoded = encode({ num: 42 });
    const v: Promise<unknown> = invoke("do_stuff_with_num", { num: 42 });
    console.log("Result of tauriless_js.encode(): ", encoded);
    v.then((result) => {
      console.log("Result of tauriless_js.invoke(): ", result);
    })
  });

  return (
    <>
      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  )
}

export default App
