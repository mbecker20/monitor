import {
  Accessor,
  Component,
  createContext,
  createResource,
  onCleanup,
  Resource,
  useContext,
} from "solid-js";
import { useLocalStorageToggle, useWindowKeyDown } from "../util/hooks";
import { getDockerAccounts, getGithubAccounts } from "../util/query";
import { USER_UPDATE } from "@monitor/util";
import { useAppDimensions } from "./DimensionProvider";
import {
  useBuilds,
  useDeployments,
  useSelected,
  useServers,
  useUpdates,
} from "./hooks";
import socket from "./socket";
import { useUser } from "./UserProvider";

export type State = {
  servers: ReturnType<typeof useServers>;
  builds: ReturnType<typeof useBuilds>;
  deployments: ReturnType<typeof useDeployments>;
  updates: ReturnType<typeof useUpdates>;
  dockerAccounts: Resource<string[] | undefined>;
  githubAccounts: Resource<string[] | undefined>;
  selected: ReturnType<typeof useSelected>;
  sidebar: {
    open: Accessor<boolean>;
    toggle: () => void;
  };
};

const context = createContext<
  State & {
    ws: ReturnType<typeof socket>;
    logout: () => void;
  }
>();

export const AppStateProvider: Component<{}> = (p) => {
  const { user, permissions, reloadUser, logout } = useUser();
  const [sidebarOpen, toggleSidebarOpen] = useLocalStorageToggle(
    "sidebar-open",
    true
  );
  const { width } = useAppDimensions();
  const [dockerAccounts] = createResource(async () =>
    permissions() >= 1 ? getDockerAccounts() : undefined
  );
  const [githubAccounts] = createResource(async () =>
    permissions() >= 1 ? getGithubAccounts() : undefined
  );
  const state: State = {
    servers: useServers(),
    builds: useBuilds(),
    deployments: useDeployments(),
    updates: useUpdates(),
    selected: useSelected(),
    dockerAccounts,
    githubAccounts,
    sidebar: {
      open: sidebarOpen,
      toggle: toggleSidebarOpen,
    },
  };

  const ws = socket(user(), state);

  onCleanup(ws.subscribe([USER_UPDATE], reloadUser));

  const resizeListener = () => {
    if (width() < 1200 && sidebarOpen()) {
      toggleSidebarOpen();
    }
  };
  addEventListener("resize", resizeListener);
  onCleanup(() => removeEventListener("resize", resizeListener));

  useWindowKeyDown((e) => {
    if (e.key === "M" && e.shiftKey) {
      toggleSidebarOpen();
    }
  });

  return (
    <context.Provider
      value={{
        ...state,
        ws,
        logout: () => {
          logout();
          ws.close();
        },
      }}
    >
      {p.children}
    </context.Provider>
  );
};

export function useAppState() {
  return useContext(context) as State & {
    ws: ReturnType<typeof socket>;
    selected: ReturnType<typeof useSelected>;
    logout: () => void;
  };
}
