import { MutableRefObject } from "react";

type Props = {
  srcRef: MutableRefObject<string>;
};

export const Editor = ({ srcRef }: Props) => {
  return <div>{srcRef.current}</div>;
};
