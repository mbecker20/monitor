import { Component, Show } from "solid-js";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import { useTheme } from "../../state/ThemeProvider";
import { useUser } from "../../state/UserProvider";
import { combineClasses } from "../../util/helpers";
import NotFound from "../NotFound";
import Grid from "../util/layout/Grid";
import Actions from "./Actions";
import { ActionStateProvider } from "./ActionStateProvider";
import Header from "./Header";
import ServerTabs from "./tabs/Tabs";
import Updates from "./Updates";

const Server: Component<{}> = (p) => {
  const { servers, selected } = useAppState();
  const server = () => servers.get(selected.id())!;
  const { themeClass } = useTheme();
  const { isMobile } = useAppDimensions();
  const { permissions, username } = useUser();
  const userCanUpdate = () => {
    if (permissions() > 1) {
      return true;
    } else if (permissions() > 0 && server()!.owners.includes(username()!)) {
      return true;
    } else {
      return false;
    }
  };
  return (
    <Show when={server()} fallback={<NotFound type="server" />}>
      <ActionStateProvider>
        <Grid class={combineClasses("content", themeClass())}>
          {/* left / actions */}
          <Grid class="left-content">
            <Header />
            <Actions />
            <Show when={!isMobile() && userCanUpdate()}>
              <Updates />
            </Show>
          </Grid>
          {/* right / tabs */}
          <ServerTabs />
        </Grid>
      </ActionStateProvider>
    </Show>
  );
};

export default Server;
