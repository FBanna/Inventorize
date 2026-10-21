import { fetchJSON } from "./util"




export async function post_id_get_file_list(
    uuid: any
) {

    let result = await fetchJSON("api/post_id_get_file_list", {
        method: "POST",

        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            component_id: uuid
        })
    })

    return result

}