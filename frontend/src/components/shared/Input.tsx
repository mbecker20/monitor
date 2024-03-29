import { Component, JSX, onMount, Show } from "solid-js";

const Input: Component<
  {
    onEdit?: (value: string) => void;
    onConfirm?: (value: string) => void;
    onEnter?: (value: string) => void;
    onEsc?: (value: string) => void;
    disabled?: boolean;
  } & JSX.InputHTMLAttributes<HTMLInputElement> &
    JSX.HTMLAttributes<HTMLDivElement>
> = (p) => {
  return (
    <Show when={!p.disabled} fallback={<div {...p}>{p.value}</div>}>
      <input
        {...p}
        onInput={(e) => p.onEdit && p.onEdit(e.currentTarget.value)}
        onBlur={
          p.onBlur || ((e) => p.onConfirm && p.onConfirm(e.currentTarget.value))
        }
        onKeyDown={
          p.onKeyDown ||
          ((e) => {
            if (e.key === "Enter") {
              p.onEnter && p.onEnter(e.currentTarget.value);
            } else if (e.key === "Escape") {
              p.onEsc ? p.onEsc(e.currentTarget.value) : e.currentTarget.blur();
            }
          })
        }
      />
    </Show>
  );
};

export default Input;

export const AutofocusInput: Component<
  {
    onEdit?: (value: string) => void;
    onConfirm?: (value: string) => void;
    onEnter?: (value: string) => void;
    onEsc?: (value: string) => void;
    disabled?: boolean;
  } & JSX.InputHTMLAttributes<HTMLInputElement> &
    JSX.HTMLAttributes<HTMLDivElement>
> = (p) => {
  let ref: HTMLInputElement;
  onMount(() => setTimeout(() => ref?.focus(), 100));
  return <Input ref={ref! as any} {...p} />;
};
