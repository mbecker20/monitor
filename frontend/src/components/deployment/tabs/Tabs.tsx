import { ContainerStatus, Log as LogType, Update } from "@monitor/types";
import {
  Component,
  createEffect,
  createSignal,
  onCleanup,
  Show,
} from "solid-js";
import Tabs from "../../util/tabs/Tabs";
import Config from "./config/Config";
import Log from "./log/Log";
import { useAppState } from "../../../state/StateProvider";
import { getDeploymentLog } from "../../../util/query";
import Icon from "../../util/Icon";
import Flex from "../../util/layout/Flex";
import {
  ADD_UPDATE,
  DELETE_CONTAINER,
  DEPLOY,
  START_CONTAINER,
  STOP_CONTAINER,
} from "@monitor/util";
import { useTheme } from "../../../state/ThemeProvider";
import { combineClasses } from "../../../util/helpers";

const DeploymentTabs: Component<{}> = () => {
  const { selected, deployments, ws } = useAppState();
  const deployment = () => deployments.get(selected.id());
  const [logTail, setLogTail] = createSignal(50);
  const [log, setLog] = createSignal<LogType>({});
  const status = () =>
    deployment()!.status === "not deployed"
      ? "not deployed"
      : (deployment()!.status as ContainerStatus).State;
  const loadLog = async () => {
    console.log("load log");
    if (deployment()?.status !== "not deployed") {
      const log = await getDeploymentLog(selected.id(), logTail());
      setLog(log);
    } else {
      setLog({});
    }
  };
  createEffect(loadLog);
  onCleanup(
    ws.subscribe([ADD_UPDATE], ({ update }: { update: Update }) => {
      if (
        update.deploymentID === selected.id() &&
        (update.operation === DEPLOY ||
          update.operation === START_CONTAINER ||
          update.operation === STOP_CONTAINER ||
          update.operation === DELETE_CONTAINER)
      ) {
        // console.log("updating log");
        setTimeout(() => {
          getDeploymentLog(selected.id()).then(setLog);
        }, 2000);
      }
    })
  );
  const { themeClass } = useTheme();
  return (
    <Show when={deployment()}>
      <Tabs
        containerClass={combineClasses("card tabs shadow", themeClass())}
        containerStyle={{ gap: "0.5rem" }}
        tabs={[
          {
            title: "config",
            element: <Config />,
          },
          status() !== "not deployed" && [
            {
              title: "log",
              element: (
                <Log
                  reload={loadLog}
                  log={log()}
                  logTail={logTail()}
                  setLogTail={setLogTail}
                />
              ),
            },
            status() !== "not deployed" && {
              title: "error log",
              titleElement: (
                <Flex gap="0.5rem" alignItems="center">
                  error log{" "}
                  <Show
                    when={
                      deployment()!.status !== "not deployed" && log().stderr
                    }
                  >
                    <Icon type="error" />
                  </Show>
                </Flex>
              ),
              element: (
                <Log
                  reload={loadLog}
                  log={log()}
                  logTail={logTail()}
                  setLogTail={setLogTail}
                  error
                />
              ),
            },
          ],
        ].flat()}
        localStorageKey="deployment-tab"
      />
    </Show>
  );
};

export default DeploymentTabs;
