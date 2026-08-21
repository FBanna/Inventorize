<template>
    <div class="popup_template menu">

        Add class

        
        <!-- <select v-model="result" multiple=false class="results">

            <option class="result" v-for="class_select in classes" :value="class_select">

                {{class_select.name}}
                
            </option>

        </select> -->

        <input class="name-input" type="text" v-model="name">

        <div class="table">

            <table>

                <thead>
                    <tr>
                        <th>name</th>
                        <th>unit</th>
                        <th>object type</th>
                    </tr>
                </thead>


                <tbody>
                    <template v-for="(field, index) in fields">

                        

                            <tr>
                                <td><input type="text" v-model="fields[index]['name']"></td>
                                <td><input type="text" v-model="fields[index]['unit']"></td>
                                <td>
                                    <select v-model="fields[index]['object_type']">

                                        <option v-for="option in object_type_options" :value="option">{{ option }}</option>

                                    </select>

                                </td>
                                <td><img class="remove" @click="remove_row(index)" src="/images/remove.svg"></td>
                            </tr>


                    
                        
                    </template>
                </tbody>

                <tr class="add">
                    <img @click="add_row" src="/images/add.svg">
                </tr>
            </table>    
        </div>





        <button class="button confirm_button" @click="confirm">Confirm</button>



        
    </div>
</template>


<script setup lang="ts">
// need `class_instance_id` & `class_name`

import { get_all_classes, post_class } from '@/api/class';
import { post_class_instance } from '@/api/class_instance';
import { pushAppError } from '@/error/error_state';
import { ref, type Ref } from 'vue';
import { clearActivePopup, opts, onSuccess } from './popup_state';

    const emptyField: Object = {
        name: "",
        unit: "",
        object_type: ""
    }
    
    const object_type_options = ["string", "integer", "float", "boolean", "datetime"]

    const name = ref()
    const fields: Ref<Array<any>> = ref([])

        


    async function confirm() {


        try {

            // await post_class_instance(
            //     result.value[0].class_id,
            //     opts.value.class_instance_id
            // )

            await post_class(
                name.value,
                fields.value
            )

            if (onSuccess != null) {
                await onSuccess.value?.()
            }

            clearActivePopup()

        } catch(e) {
            pushAppError(e)
        }

    }

    function remove_row(index: number) {

        console.log(index)
        console.log(fields.value)

        fields.value.splice(index, 1)

    }

    function add_row() {
        fields.value.push(structuredClone(emptyField))
    }

    // async function setup() {

    //     try {
    //         classes.value = await get_all_classes()

    //     } catch(e) {

    //         pushAppError(e)
    //     }
        
    // }
    



    // setup()

</script>



<style lang="scss" scoped>

    @use "@/style/import";

    .menu{
        width: 500px;
        height: 400px;

        display: grid;
        overflow: hidden;
    }

    .table {
        width: 500px;
    }


    .name-input {
        height: 20px;
        //padding-right: 100px;
        //width: 100%;
        //margin-right: 100px;
        box-sizing: border-box;
    }   

    td {
        //border: 1px black solid;
        //box-sizing: border-box;
        padding-right: 10px;
        //width: 20px;
    }

    input {
        width: 100%;
    }

    .add {
        align-items: center;
        align-content: center;
        width: 100%;
    }

</style>