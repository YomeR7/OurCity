<script lang="ts">
    import { pushNotification } from "../lib/notifications";
    import { construct, sendConstructRequest } from "../lib/construct";

    function startConstructPlacement(building: string) {
        construct.set({ status: "hover", building });
        window.addEventListener("keydown", onKey);
        window.addEventListener("contextmenu", onContext);
        window.addEventListener("pointerdown", onPointer);
    }

    function stopConstructPlacement() {
        construct.set({ status: "idle" });
        window.removeEventListener("keydown", onKey);
        window.removeEventListener("contextmenu", onContext);
        window.removeEventListener("pointerdown", onPointer);
    }

    function onKey(e: KeyboardEvent) {
        if (e.key == "Escape") stopConstructPlacement();
    }

    function onContext(e: MouseEvent) {
        e.preventDefault();
        stopConstructPlacement();
    }

    function onPointer(e: PointerEvent) {
        if (e.button == 0) {
            sendConstructRequest().then((result) => {
                if (result.ok) {
                    stopConstructPlacement();
                } else {
                    pushNotification(result.message, "red");
                }
            });
        } else if (e.button == 1) {
            stopConstructPlacement();
        }
    }
</script>

<div class="overlay">
    <button onclick={() => startConstructPlacement("House")}>New house</button>
</div>

<style>
    .overlay {
        position: absolute;
        inset: 0;
        pointer-events: none;
        padding: 1rem;
        display: flex;
        justify-content: flex-end;
        align-items: flex-start;
    }

    button {
        pointer-events: auto;
        padding: 0.45rem 0.85rem;
        border: 1px solid orange;
        border-radius: 3px;
        backdrop-filter: blur(6px);
        font: inherit;
        cursor: pointer;
    }
</style>
