
<template>
    <div class="popup_template menu">



        <img v-if="component.image" class="full-img" :src="api_url + 'api/get_image_full/' + component.component_id">
        <div v-if="!component.image"></div>


        <div class="description">
            <h1>{{ component.name }}</h1>

            <table>

                <tbody>
                    <tr>
                        <td class="title">Class</td>
                        <td>{{ class_.name }}</td>
                    </tr>

                    <tr>
                        <td class="title">Stock</td>
                        <td>{{ component.stock }}</td>
                    </tr>

                    <tr v-if="component.label != null">
                        <td class="title">Label</td>
                        <td>{{ component.label }}</td>
                    </tr>

                    <tr v-if="component.manufacturer != null">
                        <td class="title">Manufacturer</td>
                        <td>{{ component.manufacturer }}</td>
                    </tr>

                </tbody>
            </table>


            
        </div>

        <div class="attributes">

            Attributes
            <span class="attribute-span">
                <div class="class-fields" v-for="class_fields in component.attributes">
                    {{ class_fields.name }}
                    
                    <input v-model="component.attributes[class_fields.class_instance_id][field.name]" v-for="field in class_fields.fields" :type="field.object_type" :placeholder="field.name">
                </div>
            </span>
            
        </div>
        

        
    </div>
</template>


<script setup lang="ts">

import { get_all_classes, post_class, post_class_instance_id_get_class } from '@/api/class';
import { post_class_instance } from '@/api/class_instance';
import { pushAppError } from '@/error/error_state';
import { onBeforeMount, onMounted, ref, type Ref } from 'vue';
import { clearActivePopup, opts, onSuccess, onClear } from './popup_state';
import { post_label } from '@/api/label';
import router from '../router';
import { get_component } from '@/api/component';

const api_url = import.meta.env.VITE_API_URL


let class_: Ref<any> = ref({})
let component: Ref<any> = ref({})
let 



async function setup() {

    try {
        component.value = await get_component(opts.value.uuid)
        class_.value = await post_class_instance_id_get_class(component.value.class_instance_id)


        onClear.value = () => {
            router.push("/" + component.value.class_instance_id)
        }


    } catch(e: any) {
        pushAppError(e)
    }

    

    

}


setup()



</script>



<style lang="scss" scoped>

    @use "@/style/import";

    .menu{

        width: 500px;
        height: 300px;

        display: grid;
        grid-template-columns: 1fr 1fr;
        grid-template-rows: 1fr 1fr;
        column-gap: 10px;
        //grid-template-rows: 15px 20px 20px 50px;
        //row-gap: 10px;
    }

    .description {
        // background-color: red;
    }

    .full-img {
        width: 100%;
    }

    h1 {
        margin-top: 0px;
        //padding-top: 2px;
    }

    .title {
        font-weight: bold !important;
        padding-right: 10px;
    }




</style>