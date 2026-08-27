import { fetchJSON, fetchURL } from "./util"

export async function get_all_manufacturers() {

    let res: any = await fetchJSON("api/get_all_manufacturers", {
        method: "GET"
    })

    return res

}


export async function post_manufacturer(
    name: String,
    url: String
) {
                                       
    let res: any = await fetchURL("api/post_manufacturer", {
        method: "POST",

        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            name: name,
            url: url
        })
    })
    
}