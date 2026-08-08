import { ref } from "vue";


// export const active = ref<Popups | null>()
// export const opts = ref()

export const errors = ref<QueuedError[]>([])

export interface QueuedError {
    error: any,
    id: Number
}

let id = 0;

export function pushAppError(new_error: any) {
    id++
    let next = id;

    errors.value.unshift({
        error: new_error,
        id: next
    });

    setTimeout(() => removeError(id), 10000);
}

export function removeError(id: number) {
    errors.value = errors.value.filter(e => e.id !== id);
    console.log("removed")
}