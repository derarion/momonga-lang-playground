export type Stdout = string[];
export type Stderr = string[];

export type ThemeMode = "dark" | "light";
export type ThemeConfig = {
  themeMode: ThemeMode;
  toggleThemeMode: () => void;
};
export type MainLayout = "horizontal" | "vertical";

export type MonacoTheme = "monaco-theme-dark" | "monaco-theme-light";
