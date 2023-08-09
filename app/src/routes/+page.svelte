<!-- YOU CAN DELETE EVERYTHING IN THIS PAGE -->

<div class="container h-full mx-auto flex justify-center">
    <div class="space-y-10 text-center flex flex-col">
        <h2 class="h2"></h2>
        <!-- Animated Logo -->
        <!-- / -->
        <form method="POST" action="/download">
            <div class="flex justify-center space-x-2">
                <div class="input-group input-group-divider grid-cols-[auto_1fr_auto]">
                    <input type="search" placeholder="Video ID" name="videoId" value="BaW_jenozKc"/>
                    <button class="variant-filled-secondary">Download</button>
                </div>
            </div>
        </form>
        <div class="space-y-2">

        </div>
    </div>
</div>

<style lang="postcss">
    figure {
        @apply flex relative flex-col;
    }

    @keyframes glow {
        0% {
            @apply bg-primary-400/50;
        }
        33% {
            @apply bg-secondary-400/50;
        }
        66% {
            @apply bg-tertiary-400/50;
        }
        100% {
            @apply bg-primary-400/50;
        }
    }

    @keyframes pulse {
        50% {
            transform: scale(1.5);
        }
    }
</style>
