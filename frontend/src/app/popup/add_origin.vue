<template>
    <div class="popup_template menu">

        Add origin

        <input type="text" v-model="name" placeholder="name">
        <input type="text" v-model="url" placeholder="url">
        <input type="text" v-model="hurl_get" placeholder="hurl_get path">
        <input type="text" v-model="hurl_price" placeholder="hurl_price path">



        <button class="button confirm_button" @click="confirm">Confirm</button>



        
    </div>
</template>


<script setup lang="ts">

import { get_all_classes, post_class } from '@/api/class';
import { post_class_instance } from '@/api/class_instance';
import { pushAppError } from '@/error/error_state';
import { ref, type Ref } from 'vue';
import { clearActivePopup, opts, onSuccess } from './popup_state';
import { post_label } from '@/api/label';
import { post_manufacturer } from '@/api/manufacturer';
import { post_origin } from '@/api/origin';



    const name = ref()
    const url = ref()
    const hurl_get = ref()
    const hurl_price = ref()
        


    async function confirm() {


        try {

            await post_origin(
                name.value,
                url.value,
                hurl_get.value,
                hurl_price.value
            )

            if (onSuccess != null) {
                await onSuccess.value?.()
            }

            clearActivePopup()

        } catch(e: any) {
            pushAppError(e)
        }

    }

</script>



<style lang="scss" scoped>

    @use "@/style/import";

    .menu{
        width: 500px;
        height: 200px;

        display: grid;
        grid-template-rows: 15px 20px 20px 20px 20px 50px;
        row-gap: 10px;
    }

    input {
        width: 100%;
        height: 20px;
        box-sizing: border-box;
    }


</style>