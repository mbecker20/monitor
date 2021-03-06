import { Component, Show } from "solid-js";
import { BUILD, CLONE_BUILD_REPO } from "@monitor/util";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import ConfirmButton from "../util/ConfirmButton";
import Icon from "../util/Icon";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import Loading from "../util/loading/Loading";
import { useActionStates } from "./ActionStateProvider";
import { pushNotification } from "../..";
import { combineClasses } from "../../util/helpers";
import { useTheme } from "../../state/ThemeProvider";

const Actions: Component<{}> = (p) => {
  const { username, permissions } = useUser();
  const { builds, selected, ws } = useAppState();
  const build = () => builds.get(selected.id())!;
  const actions = useActionStates();
  const { themeClass } = useTheme();
  return (
    <Show when={permissions() > 1 || build().owners.includes(username()!)}>
      <Grid class={combineClasses("card shadow", themeClass())}>
        <h1>actions</h1>
        <Flex class={combineClasses("action shadow", themeClass())}>
          build{" "}
          <Show
            when={!actions.building}
            fallback={
              <button class="green">
                <Loading type="spinner" />
              </button>
            }
          >
            <ConfirmButton
              color="green"
              onConfirm={() => {
                ws.send(BUILD, { buildID: selected.id() });
                pushNotification("ok", "building...");
              }}
            >
              <Icon type="build" />
            </ConfirmButton>
          </Show>
        </Flex>
        <Flex class={combineClasses("action shadow", themeClass())}>
          reclone{" "}
          <Show
            when={!actions.cloning}
            fallback={
              <button class="orange">
                <Loading type="spinner" />
              </button>
            }
          >
            <ConfirmButton
              color="orange"
              onConfirm={() => {
                ws.send(CLONE_BUILD_REPO, { buildID: selected.id() });
                pushNotification("ok", "recloning build repo...");
              }}
            >
              <Icon type="reset" />
            </ConfirmButton>
          </Show>
        </Flex>
      </Grid>
    </Show>
  );
};

export default Actions;
