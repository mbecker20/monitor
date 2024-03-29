import {
  Component,
  createEffect,
  createSignal,
  For,
  JSX,
  JSXElement,
  Show,
} from "solid-js";
import { combineClasses } from "../../../util/helpers";
import { useToggle } from "../../../util/hooks";
import Icon from "../Icon";
import Input from "../Input";
import { Position } from "./helpers";
import Menu from "./Menu";
import s from "./menu.module.scss";

const Selector = <T,>(p: {
  selected: T;
  items: T[];
  onSelect?: (item: T, index: number) => void;
  position?: Position;
  targetClass?: string;
  targetStyle?: JSX.CSSProperties;
  menuClass?: string;
  menuStyle?: JSX.CSSProperties;
  containerStyle?: JSX.CSSProperties;
  disabled?: boolean;
  disabledClass?: string;
  disabledStyle?: JSX.CSSProperties;
  useSearch?: boolean;
  searchStyle?: JSX.CSSProperties;
  itemClass?: string;
  itemStyle?: JSX.CSSProperties;
  label?: JSXElement;
  itemMap?: (item: T) => JSXElement;
  searchItemMap?: (item: T) => string;
}) => {
  const [show, toggle] = useToggle();
  const [search, setSearch] = createSignal("");
  let search_ref: HTMLInputElement | undefined;
  const current = () => (p.itemMap ? p.itemMap(p.selected) : p.selected as JSXElement);
  createEffect(() => {
    if (show()) setTimeout(() => search_ref?.focus(), 200);
  });
  return (
    <Show
      when={!p.disabled}
      fallback={
        <div class={p.disabledClass} style={p.disabledStyle}>
          <Show when={p.label}>{p.label}</Show>
          {current()}
        </div>
      }
    >
      <Menu
        show={show()}
        close={() => {
          toggle();
          setSearch("");
        }}
        containerStyle={p.containerStyle}
        target={
          <button class={p.targetClass} onClick={toggle} style={p.targetStyle}>
            <Show when={p.label}>{p.label}</Show>
            {current()}
            <Icon type="chevron-down" />
          </button>
        }
        content={
          <>
            <Show when={p.useSearch}>
              <Input
                ref={search_ref}
                placeholder="search"
                value={search()}
                onEdit={setSearch}
                style={{ "text-align": "end", ...p.searchStyle }}
                onKeyDown={(e: any) => {
                  if (e.key === "Escape") {
                    toggle();
                    setSearch("");
                  }
                }}
              />
            </Show>
            <For
              each={
                p.useSearch
                  ? p.items.filter((item) =>
                      p.searchItemMap
                        ? p.searchItemMap(item).includes(search())
                        : p.itemMap && typeof p.itemMap(item) === "string"
                        ? (p.itemMap(item) as string).includes(search())
                        : (item as string).includes(search())
                    )
                  : p.items
              }
            >
              {(item, index) => (
                <button
                  onClick={() => {
                    p.onSelect && p.onSelect(item, index());
                    toggle();
                  }}
                  style={{
                    width: "100%",
                    "justify-content": "flex-end",
                    ...p.itemStyle,
                  }}
                  class={combineClasses(p.itemClass, s.SelectorItem)}
                >
                  {p.itemMap ? p.itemMap(item) : item as string}
                </button>
              )}
            </For>
          </>
        }
        menuClass={p.menuClass}
        menuStyle={p.menuStyle}
        position={p.position}
        padding="0.25rem"
      />
    </Show>
  );
};

export default Selector;
