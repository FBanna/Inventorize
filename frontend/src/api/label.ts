import { fetchJSON, fetchURL } from "./util"

export async function get_all_labels() {

    let res: any = await fetchJSON("api/get_all_labels", {
        method: "GET"
    })

    return res

}


export async function post_label(
    name: String,
    path: String
) {

    let res: any = await fetchURL("api/post_label", {
        method: "POST",

        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            name: name,
            path: path
        })
    })
    
}