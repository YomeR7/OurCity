<script>
    import Notifications from "./lib/components/Notifications.svelte";
    import Overlay from "./lib/components/Overlay.svelte";
    import Renderer from "./lib/components/Renderer.svelte";
</script>

<div class="stage">
    <Renderer />
    <Overlay />
    <Notifications />
</div>

<style>
    .stage {
        position: fixed;
        inset: 0;
        background: #0e1116;
    }
</style>
