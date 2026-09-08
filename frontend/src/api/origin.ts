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
    price_py: any,
    py_pn: any,
    py_qr: any
) {
    
    if (price_py == null) {
        price_py = null
    }

    if (py_pn == null) {
        py_pn = null
    }

    if (py_qr == null) {
        py_qr = null
    }

    let res: any = await fetchURL("api/post_origin", {
        method: "POST",

        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            name: name,
            url: url,
            price_py: price_py,
            py_pn: py_pn,
            py_qr: py_qr
        })
    })
    
}

export async function post_qr_python_to_origin(
    qr: any,
    origin_id: any
) {

    let res: any = await fetchJSON("api/post_qr_python_to_origin", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            qr: qr,
            origin_id: origin_id
        })
    })

}