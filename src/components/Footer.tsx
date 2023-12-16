import { Box, Link } from "@mui/material";
import { FaGithub } from "react-icons/fa";

export const Footer = () => {
  return (
    <footer>
      <Box
        sx={{
          display: "flex",
          padding: "0.5rem",
          alignItems: "center",
        }}
      >
        <FaGithub style={{ marginRight: "0.5rem" }} />
        <Link
          href={import.meta.env.VITE_APP_GITHUB_REPO_URL}
          underline="none"
          color="inherit"
        >
          <span>{import.meta.env.VITE_APP_GITHUB_REPO_NAME}</span>
        </Link>
      </Box>
    </footer>
  );
};
