import { env } from '$env/dynamic/public';

/** @type {import('./$types').Actions} */
export const actions = {
    default: async (event) => {
        const data = await event.request.formData();
        const videoUrl = data.get('videoUrl');
        await fetch(`${env.PUBLIC_API_URL}/api/download`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                videoUrl: videoUrl
            })
        });

        return {success: true};
    },
};