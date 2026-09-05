import { fetchJSON, fetchURL } from "./util"

export async function get_all_origins() {

    let res: any = await fetchJSON("api/get_all_origins", {
        method: "GET"
    })

    return res

}


export async function post_origin(
    name: String,
    url: String,
    hurl_get: any,
    hurl_price: any
) {
    
    if (hurl_get == null) {
        hurl_get = null
    }

    if (hurl_price == null) {
        hurl_price = null
    }

    let res: any = await fetchURL("api/post_origin", {
        method: "POST",

        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            name: name,
            url: url,
            hurl_get: hurl_get,
            hurl_price: hurl_price
        })
    })
    
}

export async function post_qr_hurl_to_origin(
    qr: any,
    origin_id: any
) {

    let res: any = await fetchJSON("api/post_qr_hurl_to_origin", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            qr: qr,
            origin_id: origin_id
        })
    })

}