
import { fetchJSON } from "./util";



export async function get_all_classes() {

    let res = await fetchJSON("api/get_all_classes", {
        method: "GET"
    })

    return res // catch this!

}