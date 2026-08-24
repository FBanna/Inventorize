import { fetchURL } from "./util";


export async function post_component(
    class_instance_id: any,
    name: String,
    stock: Number,
    manufacturer: String,
    label_id: any,
    attributes: any
) {

    await fetchURL("api/post_component", {
        method: "POST",

        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            class_instance_id: class_instance_id,
            name: name,
            stock: stock,
            manufacturer: manufacturer,
            label: label_id,
            attributes: attributes
        })
    })
}