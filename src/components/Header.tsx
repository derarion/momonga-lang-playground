import { useContext } from "react";

import {
  AppBar,
  Box,
  Button,
  FormControl,
  IconButton,
  InputLabel,
  MenuItem,
  Select,
  SelectChangeEvent,
  Toolbar,
} from "@mui/material";
import { IoMdDocument } from "react-icons/io";
import { MdDarkMode, MdLightMode } from "react-icons/md";
import { VscLayoutPanelOff, VscLayoutSidebarRightOff } from "react-icons/vsc";

import { snippets } from "@/constants";
import { ThemeContext } from "@/context/ThemeContext";
import { SnippetKey, UserConfig } from "@/types/types";

type Props = {
  isMuiMdScreen: boolean;
  isHorizontalLayout: boolean;
  snippetKey: SnippetKey;
  onRunClick: () => void;
  onSnippetChange: (event: SelectChangeEvent) => void;
  onMainLayoutClick: () => void;
};

export const Header = ({
  isMuiMdScreen,
  isHorizontalLayout,
  snippetKey,
  onMainLayoutClick,
  onRunClick,
  onSnippetChange,
}: Props) => {
  const { mode, toggleMode } = useContext<UserConfig>(ThemeContext);
  return (
    <AppBar
      position="static"
      sx={{ padding: "0.5rem", boxShadow: "none" }}
      color="default"
    >
      <Toolbar variant="dense">
        <Box
          sx={{
            marginRight: "auto",
            display: "flex",
            flexDirection: "row",
            alignItems: "center",
            gap: "1rem",
          }}
        >
          <Button onClick={onRunClick} variant="contained">
            Run
          </Button>
          <FormControl>
            <InputLabel id="code-snippets-select-label">
              Code Snippets
            </InputLabel>
            <Select
              labelId="code-snippets-select-label"
              id="code-snippets-select"
              label="Code Snippets"
              value={snippetKey}
              onChange={onSnippetChange}
              autoWidth
            >
              {snippets.map((snippet) => (
                <MenuItem key={snippet.key} value={snippet.key}>
                  {snippet.label}
                </MenuItem>
              ))}
            </Select>
          </FormControl>
        </Box>
        <Box
          sx={{
            marginLeft: "auto",
            display: "flex",
            flexDirection: "row",
            alignItems: "center",
          }}
        >
          <Button
            startIcon={<IoMdDocument />}
            color="inherit"
            sx={{ borderRight: 0 }}
          >
            Grammar
          </Button>
          {!isMuiMdScreen && (
            <IconButton onClick={onMainLayoutClick}>
              {isHorizontalLayout ? (
                <VscLayoutPanelOff />
              ) : (
                <VscLayoutSidebarRightOff />
              )}
            </IconButton>
          )}
          <IconButton onClick={toggleMode}>
            {mode === "light" ? <MdLightMode /> : <MdDarkMode />}
          </IconButton>
        </Box>
      </Toolbar>
    </AppBar>
  );
};
