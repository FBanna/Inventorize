<template>
    <div class="popup_template menu">

        <QrcodeStream @detect="confirm"></QrcodeStream>


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
import { QrcodeStream, type DetectedBarcode } from 'vue-qrcode-reader';


    async function confirm(detectedCodes: DetectedBarcode[]) {


        opts.value.url = detectedCodes[0]?.rawValue

        onSuccess.value?.()

        
        
        active.value = Popups.AddComponent
        




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


