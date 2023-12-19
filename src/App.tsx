import { useEffect, useRef, useState } from "react";

import { Box, Grid, useMediaQuery, useTheme } from "@mui/material";

import { Editor } from "@/components/Editor";
import { Footer } from "@/components/Footer";
import { Header } from "@/components/Header";
import { Output } from "@/components/Output";
import { Layout, Stderr, Stdout } from "@/types/types";
import init, { momonga_run } from "../momonga/pkg/momonga";

function App() {
  const srcRef = useRef<string>('print("Hello, World!");');
  const [stdout, setStdout] = useState<Stdout>([]);
  const [stderr, setStderr] = useState<Stderr>([]);

  const [userLayout, setUserLayout] = useState<Layout>(() => {
    const l = localStorage.getItem("userLayout");
    return l === "horizontal" || l === "vertical" ? l : "horizontal";
  });
  const isMuiMdScreen = useMediaQuery<boolean>(
    useTheme().breakpoints.down("md"),
  );
  const isHorizontalLayout = userLayout === "horizontal" || isMuiMdScreen;

  const handleRunClick = () => {
    setStdout([]);
    setStderr([]);
    momonga_run(srcRef.current);
  };

  const handleSrcChange = (src: string) => {
    srcRef.current = src;
  };

  const handleLayoutClick = () => {
    setUserLayout((prev) =>
      prev === "horizontal" ? "vertical" : "horizontal",
    );
  };

  useEffect(() => {
    init();

    const handlePrintStdoutEvent = (ev: Event) => {
      const event = ev as CustomEvent;
      setStdout((prev) => [...prev, event.detail]);
    };
    const handlePrintstderrEvent = (ev: Event) => {
      const event = ev as CustomEvent;
      setStderr((prev) => [...prev, event.detail]);
    };
    window.addEventListener("printstderr", handlePrintstderrEvent);
    window.addEventListener("printstdout", handlePrintStdoutEvent);

    localStorage.setItem("userLayout", userLayout);

    return () => {
      window.removeEventListener("printstdout", handlePrintStdoutEvent);
      window.removeEventListener("printstderr", handlePrintstderrEvent);
    };
  }, [userLayout]);

  return (
    <Box
      sx={{
        display: "flex",
        flexDirection: "column",
        height: "100vh",
        width: "92%",
        maxWidth: "1280px",
        margin: "0 auto",
        boxShadow: "0px 4px 4px rgba(0, 0, 0, 0.25)",
      }}
    >
      <Header
        isMuiMdScreen={isMuiMdScreen}
        isHorizontalLayout={isHorizontalLayout}
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
          direction={isHorizontalLayout ? "column" : "row"}
          spacing={0}
          sx={{ height: "100%", width: "100%" }}
        >
          <Grid
            item
            xs={8}
            sx={{
              height: isHorizontalLayout ? "calc(100% * 2 /3)" : "100%",
              width: "100%",
            }}
          >
            <Editor srcRef={srcRef} onSrcChange={handleSrcChange} />
          </Grid>
          <Grid
            item
            xs={4}
            sx={{
              height: isHorizontalLayout ? "calc(100% * 2 /3)" : "100%",
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
