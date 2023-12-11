import { useEffect } from "react";

import init, { greet } from "../momonga/pkg/momonga";

function App() {
  useEffect(() => {
    init();
  }, []);

  return (
    <>
      <button
        onClick={() => {
          greet("WebAssembly");
        }}
      >
        greet
      </button>
    </>
  );
}

export default App;
