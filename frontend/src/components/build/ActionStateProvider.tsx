import { useParams } from "@solidjs/router";
import { createContext, createEffect, onCleanup, ParentComponent, useContext } from "solid-js";
import { createStore } from "solid-js/store";
import { client } from "../..";
import { useAppState } from "../../state/StateProvider";
import { BuildActionState, Operation, UpdateStatus } from "../../types";

type State = {
  
} & BuildActionState;

const context = createContext<State>();

export const ActionStateProvider: ParentComponent<{ build_id: string }> = (p) => {
	const { ws } = useAppState();
  const [actions, setActions] = createStore<BuildActionState>({
    building: false,
	  updating: false,
  });
	createEffect(() => {
    client.get_build_action_state(p.build_id).then(setActions);
  });
  onCleanup(
    ws.subscribe([Operation.BuildBuild], (update) => {
      if (update.target.id === p.build_id) {
        setActions("building", update.status !== UpdateStatus.Complete);
      }
    })
  );
  // onCleanup(
  //   ws.subscribe([Operation.RecloneBuild], (update) => {
  //     if (update.target.id === params.id) {
  //       setActions("recloning", update.status !== UpdateStatus.Complete);
  //     }
  //   })
  // );
	// onCleanup(
  //   ws.subscribe([DELETE_BUILD], ({ complete, buildID }) => {
  //     if (buildID === selected.id()) {
  //       setActions("deleting", !complete);
  //     }
  //   })
  // );
	return (
		<context.Provider value={actions}>{p.children}</context.Provider>
	);
}

export function useActionStates() {
  return useContext(context) as State;
}