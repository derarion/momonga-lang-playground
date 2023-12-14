import { useEffect, useState } from "react";

import { Box } from "@mui/material";

import { Footer } from "@/components/Footer";
import { Header } from "@/components/Header";
import { Output } from "@/components/Output";
import { Stderr, Stdout } from "@/types/types";
import init, { greet } from "../momonga/pkg/momonga";

function App() {
  const [stdout, setStdout] = useState<Stdout>([]);
  const [stderr, setStderr] = useState<Stderr>([]);

  const handleRunClick = () => {
    greet("WebAssembly");
    setStdout(["stdout sample", "stdout sample"]);
    setStderr(["stderr sample", "stderr sample"]);
  };

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
      <Header onRunClick={handleRunClick} />
      <Box
        component="main"
        sx={{
          height: "100%",
          overflow: "hidden",
          backgroundColor: "lightblue",
        }}
      >
        <Output stdout={stdout} stderr={stderr} />
      </Box>
      <Footer />
    </Box>
  );
}

export default App;
