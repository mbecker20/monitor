import { Server } from "@monitor/types";
import { Component, Show } from "solid-js";
import { REMOVE_SERVER } from "@monitor/util";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import { combineClasses, serverStatusClass } from "../../util/helpers";
import ConfirmButton from "../util/ConfirmButton";
import Icon from "../util/Icon";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import { useTheme } from "../../state/ThemeProvider";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useLocalStorageToggle } from "../../util/hooks";
import Updates from "./Updates";

const Header: Component<{}> = (p) => {
  const { servers, selected, ws } = useAppState();
  const server = () => servers.get(selected.id()) as Server;
  const status = () =>
    server().enabled
      ? server().status === "OK"
        ? "OK"
        : "NOT OK"
      : "DISABLED";
  const { permissions, username } = useUser();
  const { themeClass } = useTheme();
  const { isMobile } = useAppDimensions();
  const [showUpdates, toggleShowUpdates] =
    useLocalStorageToggle("show-updates");
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
    <>
      <Flex
        class={combineClasses("card shadow", themeClass())}
        justifyContent="space-between"
        alignItems="center"
        style={{
          position: "relative",
          cursor: isMobile() && userCanUpdate() ? "pointer" : undefined,
        }}
        onClick={() => {
          if (isMobile() && userCanUpdate()) toggleShowUpdates();
        }}
      >
        <Grid gap="0.1rem">
          <h1>{server().name}</h1>
          <Flex gap="0.2rem" alignItems="center" style={{ opacity: 0.8 }}>
            <div>server</div>
            <Show when={server().region}>
              <Icon type="caret-right" width="0.7rem" />
              {server().region}
            </Show>
          </Flex>
        </Grid>
        <Show when={!server().isCore}>
          <Flex alignItems="center">
            <div class={serverStatusClass(status(), themeClass)}>
              {status()}
            </div>
            <Show
              when={permissions() > 1 || server().owners.includes(username())}
            >
              <ConfirmButton
                onConfirm={() => {
                  ws.send(REMOVE_SERVER, { serverID: selected.id() });
                }}
                color="red"
              >
                <Icon type="trash" />
              </ConfirmButton>
            </Show>
          </Flex>
        </Show>
        <Show when={isMobile() && userCanUpdate()}>
          <Flex gap="0.5rem" alignItems="center" class="show-updates-indicator">
            updates{" "}
            <Icon
              type={showUpdates() ? "chevron-up" : "chevron-down"}
              width="0.9rem"
            />
          </Flex>
        </Show>
      </Flex>
      <Show when={isMobile() && userCanUpdate() && showUpdates()}>
        <Updates />
      </Show>
    </>
  );
};

export default Header;
