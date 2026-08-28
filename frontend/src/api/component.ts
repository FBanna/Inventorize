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
            manufacturer_id: manufacturer,
            label_id: label_id,
            attributes: attributes
        })
    })
}



export async function post_component_with_files(
    class_instance_id: any,
    name: string,
    stock: number,
    manufacturer: string,
    label_id: any,
    attributes: any,
    files: File[],
    image: File | null
) {

    const form: FormData = new FormData();

    form.append("component", JSON.stringify({
        class_instance_id: class_instance_id,
        name: name,
        stock: stock,
        manufacturer_id: manufacturer,
        label_id: label_id,
        attributes: attributes
    }))

    for (let file of files) {
        form.append("file", file)
    }

    if (image != null) {
        form.append("image", image)
    }

    await fetchURL("api/post_component_with_files", {
        method: "POST",
        //headers: { 'Content-Type': 'multipart/form-data' },
        body: form
    })
}