import { fetchURL } from "./util";


export async function login(
    username: string,
    password: string,
    next: string | null
) {


    let res = await fetchURL("login_api", {
        method: "POST",

        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            username: username,
            password: password,
            next: next
        })
    })

    if(res.redirected) {
        window.location.href = res.url;
    }


    // else if (res.status == 401) {

    //     error.value = "incorrect login!"
    // } else if (response.status == 500) {
    //     error.value = "internal server error!"
    // }

}