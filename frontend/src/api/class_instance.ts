
import { fetchURL } from "./util";



export async function get_class_instance_descendants(
    id: any
) {

    let res = await fetchURL("api/post_id_get_class_instance_descendants", {
        method: "POST",

        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            class_instance_id: id
        })
    })

    return res.json() // catch this!

}