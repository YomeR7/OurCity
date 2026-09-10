<script lang="ts">
    import { onMount } from "svelte";
    import { renderer, loadRenderer } from "../stores/renderer";

    const canvas_id = "bevy-canvas";

    onMount(async () => {
        await loadRenderer(canvas_id);
    });
</script>

<div class="canvas-frame">
    <canvas id={canvas_id} tabindex="0"></canvas>

    {#if renderer.status == "loading"}
        <div class="veil" aria-live="polite">
            <p class="name">Our City</p>

            <div
                class="track"
                role="progressbar"
                aria-valuenow={Math.round(ratio * 100)}
            >
                <div class="fill" style:transform="scaleX({ratio})"></div>
            </div>
            <p class="status">
                {#if total}{seen} of {size} MB{:else}connecting{/if}
            </p>
        </div>
    {:else if renderer.status == "failed"}
        <div class="veil" aria-live="polite">
            <p class="name">Our City</p>

            <p class="status failed">The renderer didn't load: {error}</p>
        </div>
    {/if}
</div>

<style>
    canvas {
        outline: none;
        outline: none;
        user-select: none;
        -webkit-tap-highlight-color: transparent;
    }

    .canvas-frame {
        width: 100vw;
        height: 100vh;
    }

    .veil {
        position: absolute;
        inset: 0;
        display: grid;
        grid-template-columns: min(24rem, 78vw);
        place-content: center;
        gap: 0.85rem;
        text-align: center;
        background: var(--ground);
    }

    .status {
        font-variant-numeric: tabular-nums;
        overflow-wrap: anywhere;
    }
</style>
