import type { PageLoad } from './$types';
import { env } from "$env/dynamic/public"


export const load: PageLoad = async ({ fetch }) => {
    let response = await fetch(env.PUBLIC_BACKEND_URL);

    if (!response.ok) {
        return {
            msg: "ERROR"
        };
    }

    let data: {
        msg: string
    } = await response.json();


    return data;
};

