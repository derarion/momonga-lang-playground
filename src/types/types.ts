export type Stdout = string[];
export type Stderr = string[];

export type UserMode = "dark" | "light";
export type UserConfig = {
  mode: UserMode;
  toggleMode: () => void;
};

export type Layout = "horizontal" | "vertical";
export type MonacoTheme = "monaco-theme-dark" | "monaco-theme-light";

export type Snippet = {
  key: SnippetKey;
  label: string;
  code: string;
};
export type SnippetKey =
  | "helloWorld"
  | "arithmeticOperation"
  | "variableDeclaration"
  | "array"
  | "functionDeclarationAndCall"
  | "ifStatement"
  | "forStatement"
  | "blockScope"
  | "lexicalScoping"
  | "recursiveFunction";
