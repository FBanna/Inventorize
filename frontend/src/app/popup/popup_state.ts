import { ref } from "vue";

export enum Popups {
  AddClassInstance,
  AddClass,
  AddComponent
}


export const active = ref<Popups | null>()
export const opts = ref<any>()
export const onSuccess = ref<(() => void) | null>()



export function setActivePopup(new_active: Popups, new_opts: any, new_onSuccess: (() => void) | null) {

    if (active.value != null) {
        console.log("popup collision")
    }
    

    active.value = new_active
    opts.value = new_opts
    onSuccess.value = new_onSuccess
}

export function clearActivePopup() {
    active.value = null
    opts.value = null
    onSuccess.value = null
}