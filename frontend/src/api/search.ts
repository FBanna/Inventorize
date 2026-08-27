
import type { TablePageQuery } from "@/app/components/table/Table.vue";
import { fetchJSON, fetchURL } from "./util";



export async function post_search_get_component_with_attributes_paged(
    root: any,
    pageState: TablePageQuery,
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
            units: units,
            state: pageState
        })
    })

    return res


}

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