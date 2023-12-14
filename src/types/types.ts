export type Stdout = string[];
export type Stderr = string[];

export type ThemeMode = "dark" | "light";
export type ThemeConfig = {
  themeMode: ThemeMode;
  toggleThemeMode: () => void;
};
