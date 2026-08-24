<template>
    <div class="popup_template menu">

        Add component to {{ class_.name }}


        
        <!-- <select v-model="result" multiple=false class="results">

            <option class="result" v-for="class_select in classes" :value="class_select">

                {{class_select.name}}
                
            </option>

        </select> -->

        <!-- <input class="name-input" type="text" v-model="name"> -->
        <br>

        Core
        <span>
            <input type="text" placeholder="name" v-model="name"/>
            <input type="number" placeholder="stock" v-model="stock"/>
            <input type="text" placeholder="manufacturer" v-model="manufacturer"/>
        </span>


        Attributes
        <span class="attribute-span">
            <div class="class-fields" v-for="class_fields in fields.attributes">
                {{ class_fields.name }}
                <input v-for="field in class_fields.fields" :type="field.object_type" :placeholder="field.name">
            </div>
        </span>






        <button class="button confirm_button" @click="confirm">Confirm</button>



        
    </div>
</template>


<script setup lang="ts">
// need `class_instance_id` & `class_name`

import { get_all_classes, post_class, post_class_instance_id_get_class } from '@/api/class';
import { get_fields_from_class_instance, get_fields_from_class_instance_for_html, post_class_instance } from '@/api/class_instance';
import { pushAppError } from '@/error/error_state';
import { onBeforeMount, ref, type Ref } from 'vue';
import { clearActivePopup, opts, onSuccess } from './popup_state';
import { post_component } from '@/api/component';


    const class_: any = ref({})
    const fields: any = ref({})

    const name = ref()
    const stock = ref()
    const manufacturer = ref()

        


    async function confirm() {


        try {

            // await post_class_instance(
            //     result.value[0].class_id,
            //     opts.value.class_instance_id
            // )

            // await post_class(
            //     name.value,
            //     fields.value
            // )


            await post_component(
                opts.value.class_instance_id,
                name.value,
                stock.value,
                manufacturer.value,
                null,
                {}
            )

            if (onSuccess != null) {
                await onSuccess.value?.()
            }

            clearActivePopup()

        } catch(e) {
            pushAppError(e)
        }

    }


    async function setup() {


        console.log(opts.value.class_instance_id)



        try {
            class_.value = await post_class_instance_id_get_class(opts.value.class_instance_id)
            fields.value = await get_fields_from_class_instance_for_html(opts.value.class_instance_id)
            console.log(fields.value)

        } catch (e) {
            pushAppError(e)
        }

        
    }



    setup()

</script>



<style lang="scss" scoped>

    @use "@/style/import";

    .menu{
        width: 600px;
        height: 400px;

        display: grid;
        overflow: hidden;
    }


    input {
        width: 100%;
        height: 20px;
        margin-right: 5px;
        box-sizing: border-box;
        
    }

    span {
        display: flex;
        flex-direction: row;
        
    }

    .attribute-span {
        overflow-x: scroll;
    }

    .class-fields {
        background-color: red;
        width: 150px;
        padding: 5px;
        border-radius: 5px;
        margin-right: 5px;

    }


</style>