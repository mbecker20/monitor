import {
  Accessor,
  Component,
  createEffect,
  createSignal,
  For,
  JSX,
  JSXElement,
} from "solid-js";
import { combineClasses } from "../../../util/helpers";
import { LocalStorageSetter, useLocalStorage } from "../../../util/hooks";
import Flex from "../layout/Flex";
import s from "./tabs.module.scss";

export type Tab = {
  title: string;
  titleElement?: () => JSXElement;
  element: () => JSXElement;
};

const Tabs: Component<{
  tabs: Tab[];
  defaultSelected?: string;
  localStorageKey?: string;
  tabsGap?: string;
  tabStyle?: JSX.CSSProperties;
  containerClass?: string;
  containerStyle?: JSX.CSSProperties;
}> = (p) => {
  const def = p.defaultSelected ? p.defaultSelected : p.tabs[0].title;
  const [selected, set] = p.localStorageKey
    ? useLocalStorage(def, p.localStorageKey)
    : createSignal(def);
  createEffect(() => {
    if (p.tabs.filter((tab) => tab.title === selected())[0] === undefined) {
      set(p.tabs[0].title);
    }
  });
  return <ControlledTabs selected={selected} set={set} {...p} />;
};

export const ControlledTabs: Component<{
  tabs: Tab[];
  selected: Accessor<string>;
  set: LocalStorageSetter<string>;
  containerClass?: string;
  containerStyle?: JSX.CSSProperties;
  tabTitlesClass?: string;
  tabTitlesStyle?: JSX.CSSProperties;
  tabTitleGap?: string;
  tabTitleClass?: string;
  tabTitleStyle?: JSX.CSSProperties;
  tabContentClass?: string;
}> = (p) => {
  const current = () => p.tabs.findIndex((tab) => tab.title === p.selected());
  const getTitleClassName = (title: string) =>
    p.selected() === title
      ? combineClasses(s.TabTitle, s.Active, p.tabTitleClass)
      : combineClasses(s.TabTitle, p.tabTitleClass);
  return (
    <div
      class={combineClasses(s.Tabs, p.containerClass)}
      style={p.containerStyle}
    >
      <Flex
        class={p.tabTitlesClass}
        style={p.tabTitlesStyle}
        gap={p.tabTitleGap || "0rem"}
        alignItems="center"
        justifyContent="space-evenly"
      >
        <For each={p.tabs}>
          {(tab) => (
            <button
              class={getTitleClassName(tab.title)}
              style={p.tabTitleStyle}
              onClick={() => p.set(tab.title)}
            >
              {tab.titleElement ? tab.titleElement() : tab.title}
            </button>
          )}
        </For>
      </Flex>
      <div style={{ overflow: "hidden" }}>
        <Flex
          class={combineClasses(s.TabContent, p.tabContentClass)}
          gap="0rem"
          style={{
            width: `${p.tabs.length * 100}%`,
            left: `-${current() * 100}%`,
          }}
        >
          <For each={p.tabs}>
            {(tab) => (
              <div
                style={{
                  width: "100%",
                  // opacity: current() === i() ? 1 : 0,
                  // transition: "opacity 150ms ease",
                }}
              >
                {tab.element()}
              </div>
            )}
          </For>
        </Flex>
      </div>
    </div>
  );
};

export default Tabs;
