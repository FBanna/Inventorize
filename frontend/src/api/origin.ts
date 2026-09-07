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
    price_hurl: any,
    hurl_pn: any,
    hurl_qr: any
) {
    
    if (price_hurl == null) {
        price_hurl = null
    }

    if (hurl_pn == null) {
        hurl_pn = null
    }

    if (hurl_qr == null) {
        hurl_qr = null
    }

    let res: any = await fetchURL("api/post_origin", {
        method: "POST",

        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            name: name,
            url: url,
            price_hurl: price_hurl,
            hurl_pn: hurl_pn,
            hurl_qr: hurl_qr
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