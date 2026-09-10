import { ref } from "vue";

export enum Popups {
  AddClassInstance,
  AddClass,
  AddComponent,
  AddLabel,
  AddManufacturer,
  AddOrigin,
  ScanQR,
  Component
}


export const active = ref<Popups | null>()
export const opts = ref<any>()
export const onSuccess = ref<(() => void) | null>()
export const onClear = ref<(() => void) | null>()



export function setActivePopup(new_active: Popups, new_opts: any, new_onSuccess?: (() => void) | null, new_onClear?: (()=> void)) {

    if (active.value != null) {
        console.log("popup collision")
    }
    

    active.value = new_active
    opts.value = new_opts
    onSuccess.value = new_onSuccess
    onClear.value = new_onClear
}

export function clearActivePopup() {
    active.value = null
    opts.value = null
    onSuccess.value = null
    onClear.value?.()
}