<template>
    <div class="popup_template menu">

        <QrcodeStream @detect="confirm"></QrcodeStream>

        <QrcodeDropZone @detect="confirm">drop here</QrcodeDropZone>


        <button class="button confirm_button" @click="confirm([])">Confirm</button>



        
    </div>
</template>


<script setup lang="ts">

import { get_all_classes, post_class } from '@/api/class';
import { post_class_instance } from '@/api/class_instance';
import { pushAppError } from '@/error/error_state';
import { ref, type Ref } from 'vue';
import { clearActivePopup, opts, onSuccess, active, Popups } from './popup_state';
import { post_label } from '@/api/label';
import { QrcodeDropZone, QrcodeStream, type DetectedBarcode } from 'vue-qrcode-reader';
import { post_qr_python_to_origin } from '@/api/origin';


    async function confirm(detectedCodes: DetectedBarcode[]) {


        opts.value.url = detectedCodes[0]?.rawValue

        

        try {
            let result = await post_qr_python_to_origin(detectedCodes[0]?.rawValue, opts.value.origin_id)
            onSuccess.value?.()
            active.value = Popups.AddComponent
        } catch (e: any) {
            pushAppError(e)
        }

    

        
        
        
        




        // try {

        //     await post_label(
        //         name.value,
        //         path.value
        //     )

        //     if (onSuccess != null) {
        //         await onSuccess.value?.()
        //     }

        //     clearActivePopup()

        // } catch(e: any) {
        //     pushAppError(e)
        // }

    }

</script>



<style lang="scss" scoped>

    @use "@/style/import";

    .menu{

        width: 300px;
        height: 400px;

        // display: grid;
        // grid-template-rows: 15px 20px 20px 50px;
        // row-gap: 10px;
    }

    input {
        width: 100%;
        height: 20px;
        box-sizing: border-box;
    }


</style>


