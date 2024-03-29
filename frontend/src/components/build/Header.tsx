import { Component, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import ConfirmButton from "../shared/ConfirmButton";
import Icon from "../shared/Icon";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import { combineClasses, getId, version_to_string } from "../../util/helpers";
import { useAppDimensions } from "../../state/DimensionProvider";
import Updates from "./Updates";
import { useLocalStorageToggle } from "../../util/hooks";
import { A, useParams } from "@solidjs/router";
import { PermissionLevel } from "../../types";
import { client } from "../..";
import HoverMenu from "../shared/menu/HoverMenu";
import CopyMenu from "../CopyMenu";
import ConfirmMenuButton from "../shared/ConfirmMenuButton";

const Header: Component<{}> = (p) => {
  const { builds, servers } = useAppState();
  const params = useParams();
  const build = () => builds.get(params.id)!;
  const { user } = useUser();
  const { isSemiMobile } = useAppDimensions();
  const [showUpdates, toggleShowUpdates] =
    useLocalStorageToggle("show-updates");
  const userCanUpdate = () =>
    user().admin ||
    build().permissions![getId(user())] === PermissionLevel.Update;
  const server = () =>
    build().server_id ? servers.get(build().server_id!) : undefined;
  return (
    <>
      <Grid
        gap="0.5rem"
        class={combineClasses("card shadow")}
        style={{
          position: "relative",
          cursor: isSemiMobile() ? "pointer" : undefined,
          height: "fit-content",
        }}
        onClick={() => {
          if (isSemiMobile()) toggleShowUpdates();
        }}
      >
        <Flex alignItems="center" justifyContent="space-between">
          <h1>{build().name}</h1>
          <Show when={userCanUpdate()}>
            <Flex alignItems="center">
              <CopyMenu type="build" id={params.id} />
              <HoverMenu
                target={
                  <ConfirmMenuButton
                    onConfirm={() => {
                      client.delete_build(params.id);
                    }}
                    class="red"
                    title="delete build"
                    match={build().name}
                  >
                    <Icon type="trash" />
                  </ConfirmMenuButton>
                }
                content="delete build"
                position="bottom center"
                padding="0.5rem"
              />
            </Flex>
          </Show>
        </Flex>
        <Flex alignItems="center" justifyContent="space-between">
          <Flex alignItems="center">
            <Show when={server()} fallback={<div style={{ opacity: 0.7 }}>{build().aws_config ? "aws build" : ""}</div>}>
              <A
                href={`/server/${build().server_id}`}
                class="text-hover"
                style={{ opacity: 0.7, padding: 0 }}
              >
                {server()?.server.name}
              </A>
            </Show>
            <div style={{ opacity: 0.7 }}>build</div>
          </Flex>
          <div style={{ opacity: 0.7 }}>
            v{version_to_string(build().version)}
          </div>
        </Flex>
        <Show when={isSemiMobile()}>
          <Flex gap="0.5rem" alignItems="center" class="show-updates-indicator">
            updates{" "}
            <Icon
              type={showUpdates() ? "chevron-up" : "chevron-down"}
              width="0.9rem"
            />
          </Flex>
        </Show>
      </Grid>
      <Show when={isSemiMobile() && showUpdates()}>
        <Updates />
      </Show>
    </>
  );
};

export default Header;
