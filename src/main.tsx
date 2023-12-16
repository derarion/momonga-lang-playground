import React from "react";
import ReactDOM from "react-dom/client";

import { CssBaseline } from "@mui/material";

import App from "@/App.tsx";
import { ThemeProvider } from "@/context/ThemeProvider";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <ThemeProvider>
      <CssBaseline />
      <App />
    </ThemeProvider>
  </React.StrictMode>,
);
