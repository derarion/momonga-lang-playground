import { ReactNode, useEffect, useMemo, useState } from "react";

import { PaletteMode } from "@mui/material";
import { amber } from "@mui/material/colors";
import {
  ThemeProvider as MuiThemeProvider,
  createTheme,
} from "@mui/material/styles";

import { ThemeContext } from "@/context/ThemeContext";
import { UserConfig, UserMode } from "@/types/types";

type Props = {
  children: ReactNode;
};

export const ThemeProvider = ({ children }: Props) => {
  const [mode, setMode] = useState<UserMode>(() => {
    const l = localStorage.getItem("mode");
    return l === "dark" || l === "light" ? l : "dark";
  });

  const userConfig: UserConfig = {
    mode,
    toggleMode: () => {
      setMode((prev) => (prev === "dark" ? "light" : "dark"));
    },
  };
  const muiMode: PaletteMode = mode === "light" ? "light" : "dark";
  const muiTheme = useMemo(
    () =>
      createTheme({
        palette: {
          mode: muiMode,
          ...(muiMode === "dark"
            ? {
                primary: amber,
                text: {
                  primary: "#D4D4D4",
                },
                background: {
                  default: "#1E1E1E",
                },
              }
            : {
                primary: amber,
                text: {
                  primary: "#000000",
                },
                background: {
                  default: "#FFFFFF",
                },
              }),
        },
      }),
    [muiMode],
  );

  useEffect(() => {
    localStorage.setItem("mode", mode);
  }, [mode]);

  return (
    <ThemeContext.Provider value={userConfig}>
      <MuiThemeProvider theme={muiTheme}>{children}</MuiThemeProvider>
    </ThemeContext.Provider>
  );
};
