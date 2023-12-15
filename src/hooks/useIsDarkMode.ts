import { useContext } from "react";

import { ThemeContext } from "@/context/ThemeContext";
import { ThemeConfig } from "@/types/types";

export const useIsDarkMode = () => {
  const { themeMode } = useContext<ThemeConfig>(ThemeContext);
  return themeMode === "dark";
};
