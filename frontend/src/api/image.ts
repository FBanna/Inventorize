import { fetchJSON, fetchURL } from "./util"



export async function post_component_id_get_image_thumb(id: any) {

    let res = await fetchURL("api/post_component_id_get_image_thumb", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            component_id: id
        })
    })

    return res

}