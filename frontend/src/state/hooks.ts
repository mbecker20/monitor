import { createEffect, createResource, createSignal } from "solid-js";
import { client } from "..";
import {
  Operation,
  ServerStatus,
  SystemInformation,
  SystemStats,
  UpdateTarget,
} from "../types";
import {
  filterOutFromObj,
  getNestedEntry,
  intoCollection,
  keepOnlyInObj,
} from "../util/helpers";
import { BuildStatsResponse } from "../util/client_types";

type Collection<T> = Record<string, T>;

const groupIdPath = ["_id", "$oid"];

export function useGroups() {
  return useCollection(
    () => client.list_groups().then((res) => intoCollection(res, groupIdPath)),
    groupIdPath
  );
}

const procedureIdPath = ["_id", "$oid"];

export function useProcedures() {
  return useCollection(
    () =>
      client
        .list_procedures()
        .then((res) => intoCollection(res, procedureIdPath)),
    procedureIdPath
  );
}

const serverIdPath = ["server", "_id", "$oid"];

export function useServers() {
  return useCollection(
    () =>
      client.list_servers().then((res) => intoCollection(res, serverIdPath)),
    serverIdPath
  );
}

export function useServerStats(servers: ReturnType<typeof useServers>) {
  const [stats, set] = createSignal<Record<string, SystemStats | undefined>>(
    {}
  );
  const load = async (serverID: string) => {
    if (servers.get(serverID)?.status === ServerStatus.Ok) {
      try {
        const stats = await client.get_server_stats(serverID);
        set((s) => ({ ...s, [serverID]: stats }));
      } catch (error) {
        console.log("error getting server stats");
      }
    }
  };
  const loading: Record<string, boolean> = {};
  setTimeout(() => Object.keys(stats()).forEach(load), 30000);
  return {
    get: (serverID: string, serverStatus?: ServerStatus) => {
      const stat = stats()[serverID];
      if (
        stat === undefined &&
        !loading[serverID] &&
        (serverStatus ? serverStatus === ServerStatus.Ok : true)
      ) {
        loading[serverID] = true;
        load(serverID);
      }
      return stat;
    },
    load,
  };
}

export function useServerInfo(servers: ReturnType<typeof useServers>) {
  const [info, set] = createSignal<
    Record<string, SystemInformation | undefined>
  >({});
  const load = async (serverID: string) => {
    if (servers.get(serverID)?.status === ServerStatus.Ok) {
      try {
        const info = await client.get_server_system_info(serverID);
        set((s) => ({ ...s, [serverID]: info }));
      } catch (error) {
        console.log("error getting server info", error);
      }
    }
  };
  const loading: Record<string, boolean> = {};
  return {
    get: (serverID: string, serverStatus?: ServerStatus) => {
      const information = info()[serverID];
      if (
        information === undefined &&
        !loading[serverID] &&
        (serverStatus ? serverStatus === ServerStatus.Ok : true)
      ) {
        loading[serverID] = true;
        load(serverID);
      }
      return information;
    },
    load,
  };
}

export function useServerGithubAccounts(servers: ReturnType<typeof useServers>) {
  const [accounts, set] = createSignal<
    Record<string, string[] | undefined>
  >({});
  const load = async (serverID: string) => {
    if (servers.get(serverID)?.status === ServerStatus.Ok) {
      try {
        const info = await client.get_server_github_accounts(serverID);
        set((s) => ({ ...s, [serverID]: info }));
      } catch (error) {
        console.log("error getting server github accounts", error);
      }
    }
  };
  const loading: Record<string, boolean> = {};
  return {
    get: (serverID: string, serverStatus?: ServerStatus) => {
      const accts = accounts()[serverID];
      if (
        accts === undefined &&
        !loading[serverID] &&
        (serverStatus ? serverStatus === ServerStatus.Ok : true)
      ) {
        loading[serverID] = true;
        load(serverID);
      }
      return accts;
    },
    load,
  };
}

export function useServerDockerAccounts(
  servers: ReturnType<typeof useServers>
) {
  const [accounts, set] = createSignal<Record<string, string[] | undefined>>(
    {}
  );
  const load = async (serverID: string) => {
    if (servers.get(serverID)?.status === ServerStatus.Ok) {
      try {
        const info = await client.get_server_docker_accounts(serverID);
        set((s) => ({ ...s, [serverID]: info }));
      } catch (error) {
        console.log("error getting server docker accounts", error);
      }
    }
  };
  const loading: Record<string, boolean> = {};
  return {
    get: (serverID: string, serverStatus?: ServerStatus) => {
      const accts = accounts()[serverID];
      if (
        accts === undefined &&
        !loading[serverID] &&
        (serverStatus ? serverStatus === ServerStatus.Ok : true)
      ) {
        loading[serverID] = true;
        load(serverID);
      }
      return accts;
    },
    load,
  };
}

export function useServerSecrets(
  servers: ReturnType<typeof useServers>
) {
  const [accounts, set] = createSignal<Record<string, string[] | undefined>>(
    {}
  );
  const load = async (serverID: string) => {
    if (servers.get(serverID)?.status === ServerStatus.Ok) {
      try {
        const info = await client.get_server_available_secrets(serverID);
        set((s) => ({ ...s, [serverID]: info }));
      } catch (error) {
        console.log("error getting server github_accounts", error);
      }
    }
  };
  const loading: Record<string, boolean> = {};
  return {
    get: (serverID: string, serverStatus?: ServerStatus) => {
      const accts = accounts()[serverID];
      if (
        accts === undefined &&
        !loading[serverID] &&
        (serverStatus ? serverStatus === ServerStatus.Ok : true)
      ) {
        loading[serverID] = true;
        load(serverID);
      }
      return accts;
    },
    load,
  };
}

export function useUsernames() {
  const [usernames, set] = createSignal<Record<string, string | undefined>>({});
  const load = async (userID: string) => {
    if (userID === "github") {
      set((s) => ({ ...s, [userID]: "github" }));
    } else if (userID === "auto redeploy") {
      set((s) => ({ ...s, [userID]: "auto redeploy" }));
    } else {
      const username = await client.get_username(userID);
      set((s) => ({ ...s, [userID]: username }));
    }
  };
  const loading: Record<string, boolean> = {};
  return {
    get: (userID: string) => {
      const username = usernames()[userID];
      if (username === undefined && !loading[userID]) {
        loading[userID] = true;
        load(userID);
      }
      return username;
    },
    load,
  };
}

const buildIdPath = ["_id", "$oid"];

export function useBuilds() {
  return useCollection(
    () => client.list_builds().then((res) => intoCollection(res, buildIdPath)),
    buildIdPath
  );
}

let build_stats_loading = false;
export function useBuildStats() {
  const [stats, set] = createSignal<BuildStatsResponse>();
  const reload = () => {
    client.get_build_stats().then(set);
  };
  const get = () => {
    if (stats()) {
      return stats();
    } else if (!build_stats_loading) {
      build_stats_loading = true;
      reload()
    }
  }
  return {
    get,
    reload,
  };
}

const deploymentIdPath = ["deployment", "_id", "$oid"];

export function useDeployments() {
  const deployments = useCollection(
    () =>
      client
        .list_deployments()
        .then((res) => intoCollection(res, deploymentIdPath)),
    deploymentIdPath
  );
  const state = (id: string) => {
    const deployment = deployments.get(id)!;
    return deployment.state;
  };
  const status = (id: string) => {
    const deployment = deployments.get(id)!;
    const status = deployment.container?.status;
    return status;
  };
  return {
    ...deployments,
    status,
    state,
  };
}

export function useUpdates(target?: UpdateTarget, show_builds?: boolean) {
  const updates = useArrayWithId(
    (operations?: Operation[]) =>
      client.list_updates(0, target, show_builds, operations),
    ["_id", "$oid"]
  );
  const [noMore, setNoMore] = createSignal(false);
  const loadMore = async (operations?: Operation[]) => {
    const offset = updates.collection()?.length;
    if (offset) {
      const newUpdates = await client.list_updates(
        offset,
        target,
        show_builds,
        operations
      );
      updates.addManyToEnd(newUpdates);
      if (newUpdates.length !== 20) {
        setNoMore(true);
      }
    }
  };
  return {
    noMore,
    loadMore,
    ...updates,
  };
}

export function useArray<T, O>(
  query: (options?: O) => Promise<T[]>,
  options?: O
) {
  const [collection, set] = createSignal<T[]>();
  const load = (options?: O) => {
    query(options).then(set);
  };
  createEffect(() => {
    load(options);
  });
  const add = (item: T) => {
    set((items: T[] | undefined) => (items ? [item, ...items] : [item]));
  };
  const addManyToEnd = (items: T[]) => {
    set((curr: T[] | undefined) => (curr ? [...curr, ...items] : items));
  };
  const loaded = () => (collection() ? true : false);
  return {
    load,
    collection,
    add,
    addManyToEnd,
    loaded,
  };
}

export function useArrayWithId<T, O>(
  query: (options?: O) => Promise<T[]>,
  idPath: string[],
  options?: O
) {
  let is_loaded = false;
  const [collection, set] = createSignal<T[]>();
  const load = (_options?: O) => {
    if (!is_loaded || _options !== options) {
      query(_options).then((r) => {
        is_loaded = true;
        options = _options;
        set(r);
      });
    }
  };
  load(options);
  const addOrUpdate = (item: T) => {
    set((items: T[] | undefined) => {
      if (items) {
        const newId = getNestedEntry(item, idPath);
        const existingIndex = items.findIndex(
          (i) => getNestedEntry(i, idPath) === newId
        );
        if (existingIndex < 0) {
          return [item, ...items];
        } else {
          return items.map((e, index) => {
            if (index === existingIndex) {
              return item;
            } else {
              return e;
            }
          });
        }
      } else {
        return [item];
      }
    });
  };
  const addManyToEnd = (items: T[]) => {
    set((curr: T[] | undefined) => (curr ? [...curr, ...items] : items));
  };
  const loaded = () => (collection() ? true : false);
  const get = (id: string) => collection()?.find((item) => getNestedEntry(item, idPath) === id);
  return {
    load,
    collection,
    addOrUpdate,
    addManyToEnd,
    loaded,
    get,
  };
}

export function useCollection<T>(
  query: () => Promise<Collection<T>>,
  idPath: string[]
) {
  const [collection, { mutate, refetch }] = createResource(query);
  const add = (item: T) => {
    mutate((collection: any) => ({
      ...collection,
      [getNestedEntry(item, idPath)]: item,
    }));
  };
  const addMany = (items: Collection<T>) => {
    mutate((collection: any) => ({ ...collection, ...items }));
  };
  const del = (id: string) => {
    mutate((collection: any) => filterOutFromObj(collection, [id]));
  };
  const update = (item: T) => {
    const id = getNestedEntry(item, idPath);
    mutate((collection: any) => ({
      ...collection,
      [id]: {
        ...collection[id],
        ...item,
      },
    }));
  };
  const get = (id: string) => {
    return collection() && collection()![id];
  };
  const ids = () => collection() && Object.keys(collection()!);
  const loaded = () => (collection() ? true : false);
  const filter = (condition: (item: T) => boolean) => {
    const coll = collection();
    const _ids = coll && ids()!.filter((id) => condition(coll[id]));
    return _ids && keepOnlyInObj(coll, _ids);
  };
  const filterArray = (condition: (item: T) => boolean) => {
    const coll = collection();
    const _ids = coll && ids()!.filter((id) => condition(coll[id]));
    return _ids && _ids.map((id) => coll[id]);
  };
  return {
    collection,
    add,
    addMany,
    delete: del,
    update,
    get,
    ids,
    loaded,
    filter,
    filterArray,
    refetch,
  };
}
