import { useContext } from "react";

import { ThemeContext } from "@/context/ThemeContext";
import { UserConfig } from "@/types/types";

export const useIsDarkMode = () => {
  const { mode } = useContext<UserConfig>(ThemeContext);
  return mode === "dark";
};
