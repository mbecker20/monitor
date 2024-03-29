import { Component, createSignal, JSX } from "solid-js";

const ConfirmButton: Component<{
  onConfirm?: () => void;
  onFirstClick?: () => void;
  class?: string
  style?: JSX.CSSProperties;
  confirm?: JSX.Element;
  children: JSX.Element;
}> = (p) => {
  const [confirm, set] = createSignal(false);

  return (
    <button
      class={p.class || "green"}
      style={p.style}
      onBlur={() => set(false)}
      onClick={(e) => {
        e.stopPropagation();
        e.preventDefault();
        if (confirm()) {
          p.onConfirm && p.onConfirm();
        } else {
          p.onFirstClick && p.onFirstClick();
        }
        set((confirm) => !confirm);
      }}
    >
      {confirm() ? p.confirm || "confirm" : p.children}
    </button>
  );
};

export default ConfirmButton;
