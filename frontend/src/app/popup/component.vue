
<template>
    <div class="popup_template menu">



        <img v-if="opts.component.image" class="full-img" :src="api_url + 'api/get_image_full/' + opts.component.component_id">
        <div v-if="!opts.component.image" ></div>


        <div class="description">
            <h1>{{ opts.component.name }}</h1>

            <table>

                <tbody>
                    <tr>
                        <td class="title">Class</td>
                        <td>{{ opts.class_.name }}</td>
                    </tr>

                    <tr>
                        <td class="title">Stock</td>
                        <td>{{ opts.component.stock }}</td>
                    </tr>

                    <tr v-if="opts.component.label != null">
                        <td class="title">Label</td>
                        <td>{{ opts.component.label }}</td>
                    </tr>

                    <tr v-if="opts.component.manufacturer != null">
                        <td class="title">Manufacturer</td>
                        <td>{{ opts.component.manufacturer }}</td>
                    </tr>

                </tbody>
            </table>


            
        </div>

        

        
    </div>
</template>


<script setup lang="ts">

import { get_all_classes, post_class } from '@/api/class';
import { post_class_instance } from '@/api/class_instance';
import { pushAppError } from '@/error/error_state';
import { ref, type Ref } from 'vue';
import { clearActivePopup, opts, onSuccess } from './popup_state';
import { post_label } from '@/api/label';

const api_url = import.meta.env.VITE_API_URL


</script>



<style lang="scss" scoped>

    @use "@/style/import";

    .menu{

        width: 500px;
        height: 300px;

        display: grid;
        grid-template-columns: 1fr 1fr;
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