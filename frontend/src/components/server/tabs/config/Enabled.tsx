import { Component } from "solid-js";
import { useTheme } from "../../../../state/ThemeProvider";
import { combineClasses } from "../../../../util/helpers";
import Flex from "../../../util/layout/Flex";
import { useConfig } from "./Provider";

const Enabled: Component<{}> = (p) => {
  const { server, setServer } = useConfig();
  const { themeClass } = useTheme();
  return (
    <Flex
      class={combineClasses("config-item shadow", themeClass())}
      justifyContent="space-between"
      alignItems="center"
    >
      <h1>enabled</h1>
      <button
        class={server.enabled ? "green" : "red"}
        onClick={() => setServer("enabled", !server.enabled)}
      >
        {server.enabled ? "yes" : "no"}
      </button>
    </Flex>
  );
};

export default Enabled;
