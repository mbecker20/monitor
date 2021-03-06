import { User } from "@monitor/types";
import { Component, createEffect, createSignal, For, Show } from "solid-js";
import { pushNotification } from "../../..";
import { useTheme } from "../../../state/ThemeProvider";
import { useUser } from "../../../state/UserProvider";
import { combineClasses } from "../../../util/helpers";
import {
  addOwnerToBuild,
  getUsers,
  removeOwnerFromBuild,
} from "../../../util/query";
import ConfirmButton from "../../util/ConfirmButton";
import Input from "../../util/Input";
import Flex from "../../util/layout/Flex";
import Grid from "../../util/layout/Grid";
import Menu from "../../util/menu/Menu";
import { useConfig } from "./Provider";

const Owners: Component<{}> = (p) => {
  const { build } = useConfig();
  const { permissions, username } = useUser();
  const [userSearch, setUserSearch] = createSignal("");
  const [users, setUsers] = createSignal<User[]>([]);
  createEffect(() => {
    if (userSearch().length > 0) {
      getUsers(userSearch(), true).then((users) => {
        setUsers(users.filter((user) => !build.owners.includes(user.username)));
      });
    } else {
      setUsers([]);
    }
  });
  const { themeClass } = useTheme();
  return (
    <Show when={build.loaded}>
      <Grid class="config">
        <Grid class="config-items scroller" style={{ height: "100%" }}>
          <Grid
            class={combineClasses("config-item shadow", themeClass())}
            gap="0.5rem"
          >
            <Menu
              show={userSearch() ? true : false}
              close={() => setUserSearch("")}
              target={
                <Input
                  placeholder="add user"
                  value={userSearch()}
                  onEdit={setUserSearch}
                  style={{ width: "12rem" }}
                />
              }
              content={
                <>
                  <For each={users()}>
                    {(user) => (
                      <ConfirmButton
                        color="grey"
                        style={{
                          width: "100%",
                          "justify-content": "flex-start",
                        }}
                        onConfirm={async () => {
                          await addOwnerToBuild(build._id!, user.username);
                          pushNotification("good", "owner added to build");
                          setUserSearch("");
                        }}
                        confirm="add user"
                      >
                        {user.username}
                      </ConfirmButton>
                    )}
                  </For>
                  <Show when={users().length === 0}>no matching users</Show>
                </>
              }
              menuStyle={{ width: "12rem" }}
            />
            <For each={build.owners}>
              {(owner) => (
                <Flex
                  alignItems="center"
                  justifyContent="space-between"
                  class={combineClasses("grey-no-hover", themeClass())}
                  style={{
                    padding: "0.5rem",
                  }}
                >
                  <div class="big-text">
                    {owner}
                    {owner === username() && " ( you )"}
                  </div>
                  <Show when={permissions() > 1}>
                    <ConfirmButton
                      color="red"
                      onConfirm={async () => {
                        await removeOwnerFromBuild(build._id!, owner);
                        pushNotification(
                          "good",
                          "user removed from collaborators"
                        );
                      }}
                    >
                      remove
                    </ConfirmButton>
                  </Show>
                </Flex>
              )}
            </For>
          </Grid>
        </Grid>
      </Grid>
    </Show>
  );
};

export default Owners;
