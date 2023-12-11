import { useEffect } from "react";

import { Box } from "@mui/material";

import init, { greet } from "../momonga/pkg/momonga";

function App() {
  useEffect(() => {
    init();
  }, []);

  return (
    <Box
      sx={{
        display: "flex",
        flexDirection: "column",
        height: "100vh",
        width: "92%",
        maxWidth: "1280px",
        margin: "0 auto",
      }}
    >
      <Box component="header" sx={{ backgroundColor: "lightyellow" }}>
        header
      </Box>
      <Box
        component="main"
        sx={{
          height: "100%",
          overflow: "hidden",
          backgroundColor: "lightblue",
        }}
      >
        <button
          onClick={() => {
            greet("WebAssembly");
          }}
        >
          greet
        </button>
      </Box>
      <Box component="footer" sx={{ backgroundColor: "lightyellow" }}>
        footer
      </Box>
    </Box>
  );
}

export default App;
