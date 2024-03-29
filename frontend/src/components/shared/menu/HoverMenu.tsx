import {
  Component,
  createEffect,
  createSignal,
  JSX,
  JSXElement,
  Show,
} from "solid-js";
import { combineClasses } from "../../../util/helpers";
import Flex from "../layout/Flex";
import { getPositionClass } from "./helpers";
import { Position } from "./helpers";
import s from "./menu.module.scss";

const HoverMenu: Component<{
  target: JSXElement;
  content: JSXElement;
  position?: Position;
  padding?: string;
  contentStyle?: JSX.CSSProperties;
  contentClass?: string;
  containerStyle?: JSX.CSSProperties;
}> = (p) => {
  const [show, set] = createSignal(false);
  // const [buffer, setBuffer] = createSignal(false);
  // let timeout: number;
  // createEffect(() => {
  //   clearTimeout(timeout);
  //   if (show()) {
  //     setBuffer(true);
  //   } else {
  //     timeout = setTimeout(() => {
  //       setBuffer(false);
  //     }, 350);
  //   }
  // });
  return (
    <Flex
      class={s.HoverMenuTarget}
      style={p.containerStyle}
      onMouseEnter={() => set(true)}
      onMouseLeave={() => set(false)}
      // onClick={(e) => e.stopPropagation()}
      alignItems="center"
    >
      {p.target}
      <Show when={show()}>
        <div
          class={combineClasses(
            p.contentClass,
            getPositionClass(p.position),
            s.HoverMenu,
            // show() ? s.Enter : s.Exit,
          )}
          onMouseOut={() => {
            set(false);
          }}
          onMouseEnter={(e) => {
            set(false)
            e.stopPropagation();
          }}
          style={{ ...p.contentStyle, padding: p.padding || "0.5rem" }}
        >
          {p.content}
        </div>
      </Show>
    </Flex>
  );
};

export default HoverMenu;
