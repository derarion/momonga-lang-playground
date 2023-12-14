import { createContext } from "react";

import { ThemeConfig } from "@/types/types";

export const ThemeContext = createContext<ThemeConfig>({
  themeMode: "dark",
  toggleThemeMode: () => {},
});
