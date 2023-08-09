/** @type {import('./$types').Actions} */
export const actions = {
    default: async (event) => {
        const data = await event.request.formData();
        const videoId = data.get('videoId');

        await fetch(`http://localhost:8000/download/${videoId}`);

        return {success: true};
    },
};