<script>
    import {enhance} from '$app/forms';

    import {onMount} from 'svelte';

    onMount(() => {

        const socket = new WebSocket('ws://localhost:8000/ws');
        socket.addEventListener('open', function (event) {
            console.log("It's open");
        });

        socket.addEventListener('message', function (event) {
            console.log(event.data);
        });
    })

</script>

<div class=" flex justify-center page-container">
    <div class="container p-10 space-y-4">
        <form action="/download" method="POST" use:enhance>
            <div class="flex justify-center space-x-2">
                <div class="input-group input-group-divider grid-cols-[1fr_auto]">
                    <input type="search" placeholder="Video Url" name="videoUrl"
                           value="https://www.youtube.com/watch?v=BaW_jenozKc"/>
                    <button class="variant-filled-secondary">Download</button>
                </div>
            </div>
        </form>
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
