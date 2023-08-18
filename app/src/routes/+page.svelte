<script>
    import {enhance} from '$app/forms';
    import Item from "$lib/Item/Item.svelte";
    import {onMount} from "svelte";

    import {env} from '$env/dynamic/public';

    let items = ["test"];

    let socket;
    onMount(() => {
        socket = new WebSocket(`${env.PUBLIC_API_WS}/ws`);
        socket.addEventListener("open", ()=> {
            console.log("WS Open");
        });
        socket.addEventListener("close", ()=> {
            console.log("WS Close");
        });
        socket.addEventListener("error", ()=> {
            console.log("WS Error");
        });
        socket.addEventListener("message", (message)=> {
            console.log(`WS Message: ${message}`);
        });
    })

    function addItem() {

    }

</script>

<div class=" flex justify-center page-container">
    <div class="container p-10 space-y-4">
        <form action="/download" method="POST" use:enhance>
            <div class="flex justify-center space-x-2">
                <div class="input-group input-group-divider grid-cols-[1fr_auto]">
                    <input type="search" placeholder="Video Url" name="videoUrl"/>
                    <button class="variant-filled-secondary" on:click={addItem}>Download</button>
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
