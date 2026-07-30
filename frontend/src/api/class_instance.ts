
import { fetchJSON, fetchURL } from "./util";



export async function get_class_instance_descendants(
    id: any
) {

    let res = await fetchJSON("api/post_id_get_class_instance_descendants", {
        method: "POST",

        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            class_instance_id: id
        })
    })

    return res // catch this!

}

export async function post_class_instance(
    class_id: any,
    parent: any
    
) {

    await fetchURL("api/post_class_instance", {
        method: "POST",

        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            class_id: class_id,
            parent: parent
        })
    })


}