import { Component, createEffect, createSignal, For, onCleanup, Show } from "solid-js";
import { useUpdates } from "../../state/hooks";
import Grid from "../shared/layout/Grid";
import Update from "../update/Update";
import { useAppState } from "../../state/StateProvider";
import { combineClasses, getId } from "../../util/helpers";
import { Operation } from "../../types";
import Flex from "../shared/layout/Flex";
import Loading from "../shared/loading/Loading";
import { useParams } from "@solidjs/router";
import UpdateMenu from "../update/UpdateMenu";

const Updates: Component<{}> = (p) => {
  const { ws, deployments } = useAppState();
  const params = useParams();
  const deployment = () => deployments.get(params.id)!
  const updates = useUpdates({ type: "Deployment", id: params.id }, true);
  const buildID = () => deployment()?.deployment.build_id;
  const [openMenu, setOpenMenu] = createSignal<string | undefined>(undefined);
  let unsub = () => {};
  createEffect(() => {
    unsub();
    unsub = ws.subscribe([], (update) => {
      if (
        update.target.id === params.id ||
        (buildID() &&
          buildID() === update.target.id &&
          update.operation === Operation.BuildBuild)
      ) {
        updates.addOrUpdate(update);
      }
    });
  });
  onCleanup(() => unsub());
  return (
    <Grid class={combineClasses("card shadow")} style={{ "flex-grow": 1 }}>
      <Flex>
        <h1>updates</h1>
        <UpdateMenu
          update={openMenu() ? updates.get(openMenu()!) : undefined}
          closeMenu={() => setOpenMenu(undefined)}
        />
      </Flex>
      <Show
        when={updates.loaded()}
        fallback={
          <Flex class="full-size" alignItems="center" justifyContent="center">
            <Loading type="three-dot" scale={0.7} />
          </Flex>
        }
      >
        <Grid class="updates-container scroller">
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
              onClick={() => updates.loadMore()}
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
