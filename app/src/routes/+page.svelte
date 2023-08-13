<script>
    import {enhance} from '$app/forms';

    import {onMount} from 'svelte';

    import {io} from "socket.io-client";

    import {env} from '$env/dynamic/public';
    import Item from "$lib/Item/Item.svelte";

    onMount(() => {
        const socket = io(`${env.PUBLIC_API_WS}/client`, {
            path: '/ws/socket.io'
        });
        socket.onAny((event) => {
            console.log(event);
        });
    })
    let items = ["test", "test"];
</script>

<div class=" flex justify-center page-container">
    <div class="container p-10 space-y-4">
        <form action="/download" method="POST" use:enhance>
            <div class="flex justify-center space-x-2">
                <div class="input-group input-group-divider grid-cols-[1fr_auto]">
                    <input type="search" placeholder="Video Url" name="videoUrl"/>
                    <button class="variant-filled-secondary">Download</button>
                </div>
            </div>
        </form>
        {#each items as item}
            <Item/>
        {/each}
    </div>
</div>

<style lang="postcss">
    .page-container {
        margin-left: auto;
        margin-right: auto;
        width: 100%;
        max-width: 56rem
    }
</style>
