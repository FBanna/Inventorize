
import { fetchJSON } from "./util";



export async function get_all_classes() {

    let res = await fetchJSON("api/get_all_classes", {
        method: "GET"
    })

    return res // catch this!

}

export async function post_class_instance_id_get_class(id: any) {

    let res = await fetchJSON("api/post_class_instance_id_get_class", {
        method: "POST",

        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            class_instance_id: id
        })
    })

    return res

}