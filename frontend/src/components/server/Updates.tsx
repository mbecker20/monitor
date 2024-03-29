import {
  Component,
  createEffect,
  createSignal,
  For,
  onCleanup,
  Show,
} from "solid-js";
import { useUpdates } from "../../state/hooks";
import { useAppState } from "../../state/StateProvider";
import Update from "../update/Update";
import Grid from "../shared/layout/Grid";
import { combineClasses, getId } from "../../util/helpers";
import { useParams } from "@solidjs/router";
import Flex from "../shared/layout/Flex";
import UpdateMenu from "../update/UpdateMenu";
import Loading from "../shared/loading/Loading";

const Updates: Component<{}> = (p) => {
  const { ws } = useAppState();
  const params = useParams();
  const updates = useUpdates({ type: "Server", id: params.id });
  const [openMenu, setOpenMenu] = createSignal<string | undefined>(undefined);
  let unsub = () => {};
  createEffect(() => {
    unsub();
    unsub = ws.subscribe([], (update) => {
      if (update.target.type === "Server" && update.target.id === params.id) {
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
