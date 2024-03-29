import { useParams } from "@solidjs/router";
import { Component, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { useUser } from "../../../state/UserProvider";
import { ServerStatus } from "../../../types";
import SimpleTabs from "../../shared/tabs/SimpleTabs";
import { Tab } from "../../shared/tabs/Tabs";
import Config from "./config/Config";
import { ConfigProvider } from "./config/Provider";
import Info from "./Info";
import Permissions from "./Permissions";
import Images from "./Images";

const ServerTabs: Component<{}> = (p) => {
  const { servers } = useAppState();
  const params = useParams();
  const { user } = useUser();
  const server = () => servers.get(params.id)!;
  return (
    <Show when={server()}>
      <ConfigProvider>
        <SimpleTabs
          containerClass="card shadow"
          containerStyle={{ width: "100%", "box-sizing": "border-box" }}
          tabs={
            [
              {
                title: "config",
                element: () => <Config />,
              },
              server()?.status === ServerStatus.Ok && {
                title: "info",
                element: () => <Info />
              },
              server()?.status === ServerStatus.Ok && {
                title: "images",
                element: () => <Images />
              },
              user().admin && {
                title: "permissions",
                element: () => <Permissions />,
              },
            ].filter((e) => e) as Tab[]
          }
          localStorageKey="server-tab"
        />
      </ConfigProvider>
    </Show>
  );
};

export default ServerTabs;
