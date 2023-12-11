import { useEffect } from "react";

import { Box } from "@mui/material";

import init, { greet } from "../momonga/pkg/momonga";
import { Footer } from "@/components/Footer";
import { Header } from "@/components/Header";

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
      <Header />
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
      <Footer />
    </Box>
  );
}

export default App;
