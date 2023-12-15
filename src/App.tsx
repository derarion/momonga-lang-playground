import { useEffect, useRef, useState } from "react";

import { Box, Grid } from "@mui/material";

import { Editor } from "@/components/Editor";
import { Footer } from "@/components/Footer";
import { Header } from "@/components/Header";
import { Output } from "@/components/Output";
import { MainLayout, Stderr, Stdout } from "@/types/types";
import init, { greet } from "../momonga/pkg/momonga";

function App() {
  const srcRef = useRef<string>("Hello, World!");
  const [stdout, setStdout] = useState<Stdout>([]);
  const [stderr, setStderr] = useState<Stderr>([]);

  const [mainLayout, setMainLayout] = useState<MainLayout>(() => {
    const l = localStorage.getItem("mainLayout");
    return l === "horizontal" || l === "vertical" ? l : "horizontal";
  });
  const isMainLayoutHorizontal = mainLayout === "horizontal";

  const handleRunClick = () => {
    greet("WebAssembly");
    setStdout(["stdout sample", "stdout sample"]);
    setStderr(["stderr sample", "stderr sample"]);
  };

  const handleLayoutClick = () => {
    setMainLayout((prev) =>
      prev === "horizontal" ? "vertical" : "horizontal",
    );
  };

  useEffect(() => {
    init();
    localStorage.setItem("mainLayout", mainLayout);
  }, [mainLayout]);

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
      <Header
        isMainLayoutHorizontal={isMainLayoutHorizontal}
        onRunClick={handleRunClick}
        onMainLayoutClick={handleLayoutClick}
      />
      <Box
        component="main"
        sx={{
          height: "100%",
          width: "100%",
          overflow: "hidden",
        }}
      >
        <Grid
          container
          direction={isMainLayoutHorizontal ? "column" : "row"}
          spacing={0}
          sx={{ height: "100%", width: "100%" }}
        >
          <Grid
            item
            xs={8}
            sx={{
              height: isMainLayoutHorizontal ? "calc(100% * 2 /3)" : "100%",
              width: "100%",
            }}
          >
            <Editor srcRef={srcRef} />
          </Grid>
          <Grid
            item
            xs={4}
            sx={{
              height: isMainLayoutHorizontal ? "calc(100% * 2 /3)" : "100%",
              width: "100%",
              overflow: "auto",
            }}
          >
            <Output stdout={stdout} stderr={stderr} />
          </Grid>
        </Grid>
      </Box>
      <Footer />
    </Box>
  );
}

export default App;
