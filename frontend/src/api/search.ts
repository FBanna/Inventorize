
import { fetchJSON, fetchURL } from "./util";




export async function post_search_get_component_with_attributes(
    root: any,
    units: any
    
) {

    if (root == null) {
        root = null
    }
    
    let res = await fetchJSON("api/post_search_get_component_with_attributes", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            root: root,
            units: units
        })
    })

    return res


}

export async function post_search_get_facets(
    root: any,
    units: any
) {

    let res = await fetchJSON("api/post_search_get_facets", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            root: root,
            units: units
        })
    })

    return res

}