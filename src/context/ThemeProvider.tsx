import { ReactNode, useEffect, useMemo, useState } from "react";

import { PaletteMode } from "@mui/material";
import { amber } from "@mui/material/colors";
import {
  ThemeProvider as MuiThemeProvider,
  createTheme,
} from "@mui/material/styles";

import { ThemeContext } from "@/context/ThemeContext";
import { ThemeConfig, ThemeMode } from "@/types/types";

type Props = {
  children: ReactNode;
};

export const ThemeProvider = ({ children }: Props) => {
  const [themeMode, setThemeMode] = useState<ThemeMode>(() => {
    const themeMode = localStorage.getItem("themeMode");
    return themeMode === "dark" || themeMode === "light" ? themeMode : "dark";
  });

  const themeConfig: ThemeConfig = {
    themeMode,
    toggleThemeMode: () => {
      setThemeMode((prev) => (prev === "dark" ? "light" : "dark"));
    },
  };
  const muiMode: PaletteMode = themeMode === "light" ? "light" : "dark";
  const muiTheme = useMemo(
    () =>
      createTheme({
        palette: {
          mode: muiMode,
          ...(muiMode === "dark"
            ? {
                primary: amber,
                text: {
                  primary: "#fff",
                },
              }
            : {
                primary: amber,
                text: {
                  primary: "#000",
                },
              }),
        },
      }),
    [muiMode],
  );

  useEffect(() => {
    localStorage.setItem("themeMode", themeMode);
  }, [themeMode]);

  return (
    <ThemeContext.Provider value={themeConfig}>
      <MuiThemeProvider theme={muiTheme}>{children}</MuiThemeProvider>
    </ThemeContext.Provider>
  );
};
