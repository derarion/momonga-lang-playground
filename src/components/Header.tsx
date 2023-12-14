import {
  AppBar,
  Box,
  Button,
  FormControl,
  IconButton,
  InputLabel,
  MenuItem,
  Select,
  Toolbar,
} from "@mui/material";
import { IoMdDocument } from "react-icons/io";
import { MdDarkMode } from "react-icons/md";
import { VscLayoutPanelOff } from "react-icons/vsc";

type Props = {
  onRunClick: () => void;
};

export const Header = ({ onRunClick }: Props) => {
  return (
    <AppBar position="static" sx={{ padding: "0.5rem" }}>
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
            >
              <MenuItem>Item1</MenuItem>
              <MenuItem>Item2</MenuItem>
              <MenuItem>Item3</MenuItem>
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
          <IconButton>
            <VscLayoutPanelOff />
          </IconButton>
          <IconButton>
            <MdDarkMode />
          </IconButton>
        </Box>
      </Toolbar>
    </AppBar>
  );
};
