import { Box, Divider } from "@mui/material";

import { Stderr, Stdout } from "@/types/types";

type Props = {
  stdout: Stdout;
  stderr: Stderr;
};

export const Output = ({ stdout, stderr }: Props) => {
  return (
    <Box>
      <Box>
        <Divider>Standard Output</Divider>
        <Box>
          <pre style={{ margin: 0, lineHeight: 1.15 }}>{stdout.join("\n")}</pre>
        </Box>
      </Box>
      <Box>
        <Divider>Standard Error</Divider>
        <Box>
          <pre style={{ margin: 0, lineHeight: 1.15 }}>{stderr.join("\n")}</pre>
        </Box>
      </Box>
    </Box>
  );
};
