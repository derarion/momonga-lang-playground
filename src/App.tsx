import { useCallback, useEffect, useRef, useState } from "react";

import {
  Box,
  Grid,
  SelectChangeEvent,
  useMediaQuery,
  useTheme,
} from "@mui/material";

import { Editor } from "@/components/Editor";
import { Footer } from "@/components/Footer";
import { Header } from "@/components/Header";
import { Output } from "@/components/Output";
import { snippets } from "@/constants";
import { Layout, SnippetKey, Stderr, Stdout } from "@/types/types";
import init, {
  is_momonga_parse_error,
  momonga_run,
} from "../momonga/pkg/momonga";

function App() {
  const isWasmIntitializedRef = useRef<boolean>(false);

  const srcRef = useRef<string>("");
  const [isParseError, setIsParseError] = useState<boolean>(false);
  const [stdout, setStdout] = useState<Stdout>([]);
  const [stderr, setStderr] = useState<Stderr>([]);
  const [snippetKey, setSnippetKey] = useState<SnippetKey>(snippets[0].key);

  const [userLayout, setUserLayout] = useState<Layout>(() => {
    const l = localStorage.getItem("userLayout");
    return l === "horizontal" || l === "vertical" ? l : "horizontal";
  });
  const isMuiMdScreen = useMediaQuery<boolean>(
    useTheme().breakpoints.down("md"),
  );
  const isHorizontalLayout = userLayout === "horizontal" || isMuiMdScreen;

  const handleRunClick = useCallback(() => {
    setStdout([]);
    setStderr([]);
    momonga_run(srcRef.current); // NOTE: In order to run on Worker, it is necessary to change the way of passing its output data to main thread.
  }, []);

  const handleSrcChange = useCallback((src: string) => {
    srcRef.current = src;

    if (!isWasmIntitializedRef.current) return;
    setIsParseError(is_momonga_parse_error(src));
  }, []);

  const handleLayoutClick = useCallback(() => {
    setUserLayout((prev) =>
      prev === "horizontal" ? "vertical" : "horizontal",
    );
  }, []);

  const handleSnippetChange = useCallback(
    (event: SelectChangeEvent<string>) => {
      const snippet = snippets.find(
        (snippet) => snippet.key === event.target.value,
      );
      if (snippet) {
        setSnippetKey(snippet.key);
      }
    },
    [],
  );

  useEffect(() => {
    (async () => {
      await init();
      isWasmIntitializedRef.current = true;
    })();

    const handleStdoutEvent = (ev: Event) => {
      const event = ev as CustomEvent;
      setStdout((prev) => [...prev, event.detail]);
    };
    const handleStderrEvent = (ev: Event) => {
      const event = ev as CustomEvent;
      setStderr((prev) => [...prev, event.detail]);
    };
    window.addEventListener("stderr", handleStderrEvent);
    window.addEventListener("stdout", handleStdoutEvent);

    return () => {
      window.removeEventListener("stdout", handleStdoutEvent);
      window.removeEventListener("stderr", handleStderrEvent);
    };
  }, []);

  useEffect(() => {
    localStorage.setItem("userLayout", userLayout);
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
        snippetKey={snippetKey}
        onRunClick={handleRunClick}
        onSnippetChange={handleSnippetChange}
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
            <Editor
              isParseError={isParseError}
              srcRef={srcRef}
              snippetKey={snippetKey}
              onSrcChange={handleSrcChange}
            />
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
