import { A } from "@solidjs/router";
import { Component, createEffect, createSignal, For, Show } from "solid-js";
import { OPERATIONS } from "../../..";
import { useAppState } from "../../../state/StateProvider";
import { Operation } from "../../../types";
import Flex from "../../shared/layout/Flex";
import Grid from "../../shared/layout/Grid";
import Loading from "../../shared/loading/Loading";
import Selector from "../../shared/menu/Selector";
import Update from "./Update";
import UpdateMenu from "../../update/UpdateMenu";
import { getId } from "../../../util/helpers";

const Updates: Component<{}> = () => {
  const { updates } = useAppState();
  const [operation, setOperation] = createSignal<Operation>();
  createEffect(() => {
    if (operation()) {
      updates.load([operation()!]);
    } else {
      updates.load();
    }
  });
  const [openMenu, setOpenMenu] = createSignal<string | undefined>(undefined);
  return (
    <Grid class="card shadow" style={{ "flex-grow": 1 }}>
      <Flex alignItems="center" justifyContent="space-between">
        <Flex>
          <A href="/updates" style={{ padding: 0 }}>
            <h1>updates</h1>
          </A>
          <UpdateMenu
            update={openMenu() ? updates.get(openMenu()!) : undefined}
            closeMenu={() => setOpenMenu(undefined)}
          />
        </Flex>
        <Selector
          label="operation: "
          selected={operation() ? operation()! : "all"}
          items={["all", ...OPERATIONS]}
          onSelect={(o) =>
            o === "all"
              ? setOperation(undefined)
              : setOperation(o.replaceAll(" ", "_") as Operation)
          }
          targetStyle={{ padding: "0" }}
          position="bottom right"
          searchStyle={{ width: "15rem" }}
          menuClass="scroller"
          menuStyle={{ "max-height": "50vh" }}
          useSearch
        />
      </Flex>
      <Show
        when={updates.loaded()}
        fallback={
          <Flex justifyContent="center">
            <Loading type="three-dot" scale={0.7} />
          </Flex>
        }
      >
        <Grid class="updates-container-small scroller">
          <For each={updates.collection()}>
            {(update) => (
              <Update
                update={update}
                openMenu={() => setOpenMenu(getId(update))}
              />
            )}
          </For>
          <Show when={!updates.noMore()}>
            <button
              class="grey"
              style={{ width: "100%" }}
              onClick={() =>
                operation()
                  ? updates.loadMore([operation()!])
                  : updates.loadMore()
              }
            >
              load more
            </button>
          </Show>
        </Grid>
      </Show>
    </Grid>
  );
};

export default Updates;
