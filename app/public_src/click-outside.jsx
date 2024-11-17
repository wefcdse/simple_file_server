import { onCleanup } from "solid-js";

export default function clickOutside(el, accessor) {
    // console.log(accessor());
    const onClick = (e) => {
        // console.log(e);
        return !el.contains(e.target) && accessor()?.()
    };
    document.body.addEventListener("click", onClick);
    onCleanup(() => document.body.removeEventListener("click", onClick));
}
