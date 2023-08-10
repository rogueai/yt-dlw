/** @type {import('./$types').Actions} */
export const actions = {
    default: async (event) => {
        const data = await event.request.formData();
        const videoUrl = data.get('videoUrl');

        await fetch('http://localhost:8000/download/', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                url: videoUrl
            })
        });

        return {success: true};
    },
};