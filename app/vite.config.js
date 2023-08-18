import {sveltekit} from '@sveltejs/kit/vite';
import {defineConfig} from 'vitest/config';

// export default defineConfig(({command, mode}) => {
//     // Load env file based on `mode` in the current working directory.
//     // Set the third parameter to '' to load all env regardless of the `VITE_` prefix.
//     const env = loadEnv(mode, process.cwd(), '') // here Deno.env
//     return {
//         // vite config
//         define: {
//             __APP_ENV__: JSON.stringify(env.APP_ENV),
//             plugins: [sveltekit()],
//             test: {
//                 include: ['src/**/*.{test,spec}.{js,ts}']
//             }
//         },
//     }
// })

export default defineConfig({
    plugins: [sveltekit()],
    test: {
        include: ['src/**/*.{test,spec}.{js,ts}']
    }
});
