import {
  Accessor,
  Component,
  createEffect,
  createSignal,
  JSX,
  JSXElement,
  Show,
} from "solid-js";
import { combineClasses } from "../../../util/helpers";
import { useKeyDown } from "../../../util/hooks";
import Icon from "../icons/Icon";
import Flex from "../layout/Flex";
import Grid from "../layout/Grid";
import s from "./Menu.module.css";

const CenterMenu: Component<{
  show: Accessor<boolean>;
  toggleShow: () => void;
  content: JSXElement;
  target: JSXElement;
  targetStyle?: JSX.CSSProperties;
  targetClass?: string;
  title?: string;
  padding?: string | number;
  style?: JSX.CSSProperties;
}> = (p) => {
  const [buffer, set] = createSignal(p.show());
  createEffect(() => {
    if (p.show()) {
      set(true);
    } else {
      setTimeout(() => {
        set(false);
      }, 250);
    }
  });
  return (
    <>
      <button
        onClick={p.toggleShow}
        class={p.targetClass}
        style={p.targetStyle}
      >
        {p.target}
      </button>
      <Show when={buffer()}>
        <Child {...p} show={p.show} toggleShow={p.toggleShow} />
      </Show>
    </>
  );
};

const Child: Component<{
  title?: string;
  content: JSXElement;
  show: Accessor<boolean>;
  toggleShow: () => void;
  padding?: string | number;
  style?: JSX.CSSProperties;
}> = (p) => {
  useKeyDown("Escape", p.toggleShow);
  return (
    <Grid
      class={combineClasses(s.CenterMenuContainer, p.show() ? s.Enter : s.Exit)}
      onClick={p.toggleShow}
    >
      <Grid
        class={combineClasses(s.Menu, "shadow")}
        style={{ padding: p.padding || "1rem", ...p.style }}
        onClick={(e) => e.stopPropagation()}
      >
        <Flex class={s.CenterMenuHeader} gap="3rem" justifyContent="space-between" alignItems="center">
          <div class={s.CenterMenuTitle}>{p.title}</div>
          <button class="red" onClick={p.toggleShow}>
            <Icon type="cross" />
          </button>
        </Flex>
        {p.content}
      </Grid>
    </Grid>
  );
};

export default CenterMenu;
