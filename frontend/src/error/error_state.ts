import type { StatusError } from "@/api/util";
import { ref } from "vue";


// export const active = ref<Popups | null>()
// export const opts = ref()

export const errors = ref<QueuedError[]>([])

export interface QueuedError {
    error: StatusError,
    id: number,
    count: number,
    timer_id: number
}

let id = 0;

export function pushAppError(new_error: StatusError) {

    for (let error of errors.value) {
        if (error.error.message == new_error.message) {

            error.count += 1
            
            clearTimeout(error.timer_id)

            error.timer_id = setTimeout(() => {
                    removeError(error.id)
                },
                10000
            )

            return



        }
    }

    id++;


    errors.value.unshift({
        error: new_error,
        id: id,
        count: 1,
        timer_id: setTimeout(() => removeError(id), 10000)
    });


    

}

export function removeError(id: number) {
    errors.value = errors.value.filter(e => e.id !== id);
}