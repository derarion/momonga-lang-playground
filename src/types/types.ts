export type Stdout = string[];
export type Stderr = string[];

export type UserMode = "dark" | "light";
export type UserConfig = {
  mode: UserMode;
  toggleMode: () => void;
};

export type Layout = "horizontal" | "vertical";
export type MonacoTheme = "monaco-theme-dark" | "monaco-theme-light";
