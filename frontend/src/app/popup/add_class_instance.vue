<template>
    <div class="popup_template menu">

        Add class instance to {{ opts.class_name }}


        <select v-model="result" multiple=false class="results">

            <option class="result" v-for="class_select in classes" :value="class_select">

                {{class_select.name}}
                
            </option>

        </select>



        <button class="button confirm_button" @click="confirm">Confirm</button>



        
    </div>
</template>


<script setup lang="ts">
// need `class_instance_id` & `class_name`

import { get_all_classes } from '@/api/class';
import { post_class_instance } from '@/api/class_instance';
import { pushAppError } from '@/error/error_state';
import { ref } from 'vue';
import { clearActivePopup, opts, onSuccess } from './popup_state';



    const classes = ref()
    const result = ref<any[]>([])


    async function confirm() {


        try {

            await post_class_instance(
                result.value[0].class_id,
                opts.value.class_instance_id
            )

            if (onSuccess != null) {
                await onSuccess.value?.()
            }

            clearActivePopup()

        } catch(e: any) {
            pushAppError(e)
        }

    }

    async function setup() {

        try {
            classes.value = await get_all_classes()

        } catch(e: any) {

            pushAppError(e)
        }
        
    }
    



    setup()

</script>



<style lang="scss" scoped>

    @use "@/style/import";

    .menu{
        width: 300px;
        height: 200px;


        display: grid;
        grid-template-rows: 15px auto 50px;
        row-gap: 10px;
    }


    .class_selection {

        margin-top: 3px;
        margin-bottom: 3px;
        width: 100%;
        height: calc(100% - 45px);
        box-sizing: border-box;
        border: none;
        outline: none;

        white-space: nowrap;
        overflow-y: auto;
        overflow-x: hidden;
    
    
    }

    .class_select {
        font-weight: normal;
        height: 15pt;

    }

    .confirm_button {
        align-self: right;
    }

</style>