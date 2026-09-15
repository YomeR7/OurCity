<script lang="ts">
    import { fly } from "svelte/transition";
    import { notifications } from "../lib/notifications";

    let notifs = [];
    notifications.subscribe((val) => {
        notifs = val;
    });
</script>

<ul class="notifications">
    {#each notifs as notif (notif.id)}
        <li
            in:fly={{ x: -8, duration: 140 }}
            out:fly={{ x: -8, duration: 300 }}
            style:color={notif.color}
        >
            {notif.text}
        </li>
    {/each}
</ul>

<style>
    .notifications {
        position: absolute;
        left: 1rem;
        bottom: 1rem;
        margin: 0;
        padding: 0;
        max-width: min(32rem, 45vw);
        list-style: none;
        display: flex;
        flex-direction: column;
        gap: 0.15rem;
        pointer-events: none;
    }

    li {
        padding: 0.15rem 0.45rem;
        border-radius: 2px;
        background: color-mix(in srgb, var(--ground) 70%, transparent);
        font-family: ui-monospace, "SF Mono", Menlo, monospace;
        font-size: var(--step--1);
        line-height: 1.45;
        overflow-wrap: anywhere;
    }
</style>
