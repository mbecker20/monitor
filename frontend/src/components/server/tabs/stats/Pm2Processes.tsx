import { PM2Process } from "@monitor/types";
import { Component, createEffect, createSignal, For, Show } from "solid-js";
import { pushNotification } from "../../../..";
import { useAppState } from "../../../../state/StateProvider";
import { useTheme } from "../../../../state/ThemeProvider";
import { combineClasses } from "../../../../util/helpers";
import { getServerPm2Processes } from "../../../../util/query";
import Button from "../../../util/Button";
import Icon from "../../../util/Icon";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import Loading from "../../../util/loading/Loading";
import s from "./stats.module.scss";

const Pm2Processes: Component<{}> = (p) => {
  const { selected } = useAppState();
  const [pm2Proc, setPm2Proc] = createSignal<PM2Process[]>();
  const [refreshing, setRefreshing] = createSignal(false);
  const loadPm2 = () => {
    if (selected.id()) {
      getServerPm2Processes(selected.id()).then(setPm2Proc);
    }
  };
  createEffect(loadPm2);
  const { themeClass } = useTheme();
  return (
    <Show when={pm2Proc() && pm2Proc()!.length > 0}>
      <Grid class={combineClasses(s.StatsContainer, themeClass())}>
        <Flex justifyContent="space-between" alignItems="center">
          <h1>pm2 processes</h1>
          <Button
            class="blue"
            onClick={async () => {
              setRefreshing(true);
              const processes = await getServerPm2Processes(selected.id());
              setPm2Proc(processes);
              setRefreshing(false);
              pushNotification("good", "processes refreshed");
            }}
          >
            <Show when={!refreshing()} fallback={<Loading />}>
              <Icon type="refresh" />
            </Show>
          </Button>
        </Flex>
        <Grid style={{ padding: "0.5rem" }}>
          <For each={pm2Proc()}>
            {(process) => (
              <Flex justifyContent="space-between" alignItems="center">
                <h2>{process.name}</h2>
                <Flex alignItems="center">
                  <div>{process.status}</div>
                  <div>cpu: {process.cpu}%</div>
                  <div>
                    mem:{" "}
                    {process.memory
                      ? `${process.memory / 1024000} mb`
                      : "unknown"}
                  </div>
                </Flex>
              </Flex>
            )}
          </For>
        </Grid>
      </Grid>
    </Show>
  );
};

export default Pm2Processes;
