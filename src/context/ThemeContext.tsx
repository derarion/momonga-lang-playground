import { createContext } from "react";

import { UserConfig } from "@/types/types";

export const ThemeContext = createContext<UserConfig>({
  mode: "dark",
  toggleMode: () => {},
});
