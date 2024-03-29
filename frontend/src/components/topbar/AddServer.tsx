import { Component, JSX, onMount } from "solid-js";
import { createStore } from "solid-js/store";
import { client, pushNotification } from "../..";
import { CreateServerBody } from "../../util/client_types";
import { useToggle } from "../../util/hooks";
import Icon from "../shared/Icon";
import Input from "../shared/Input";
import Grid from "../shared/layout/Grid";
import CenterMenu from "../shared/menu/CenterMenu";

const AddServer: Component<{}> = () => {
  const [show, toggleShow] = useToggle();
  return (
    <CenterMenu
      show={show}
      toggleShow={toggleShow}
      target="add server"
      title="add server"
      targetClass="green shadow"
      content={() => <Content close={toggleShow} />}
      style={{ "box-sizing": "border-box", "max-width": "90vw" }}
      position="center"
    />
  );
};

const INPUT_STYLE: JSX.CSSProperties = {
  "font-size": "1.25rem",
  width: "500px",
  "max-width": "80vw",
};

const Content: Component<{ close: () => void }> = (p) => {
  let nameInput: HTMLInputElement | undefined;
  const [server, setServer] = createStore<CreateServerBody>({
    name: "",
    address: "",
  });
  onMount(() => nameInput?.focus());
  const create = async () => {
    if (server.name.length > 0 && server.address.length > 0) {
      await client.create_server(server);
      p.close();
    } else {
      pushNotification("bad", "a field is empty. fill in all fields");
    }
  };
  return (
    <Grid
      placeItems="center"
      style={{
        
      }}
    >
      <Input
        class="darkgrey"
        ref={nameInput}
        value={server.name}
        onEdit={(name) => setServer("name", name)}
        placeholder="name"
        style={INPUT_STYLE}
      />
      <Input
        class="darkgrey"
        value={server.address}
        onEdit={(address) => setServer("address", address)}
        placeholder="address"
        style={INPUT_STYLE}
      />
      <button class="green" style={{ width: "100%" }} onClick={create}>
        add
      </button>
    </Grid>
  );
};

export default AddServer;
