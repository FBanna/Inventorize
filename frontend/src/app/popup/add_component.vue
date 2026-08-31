<template>
    <div class="popup_template menu">

        <div class="row">
            Add component to {{ class_.name }}
        </div>

        


        
        <!-- <select v-model="result" multiple=false class="results">

            <option class="result" v-for="class_select in classes" :value="class_select">

                {{class_select.name}}
                
            </option>

        </select> -->

        <!-- <input class="name-input" type="text" v-model="name"> -->
        
        <div class="row">

            Core
            <span>
                <input class="core-input" type="text" placeholder="name" v-model="name"/>
                <input class="core-input" type="number" placeholder="stock" v-model="stock"/>

                <select v-model="manufacturer">

                    <option v-for="man in manufacturer_options" :value="man.manufacturer_id">

                        {{man.name}}
                        
                    </option>

                    <option :value="null">None</option>

                </select>

                <select v-model="label">

                    <option v-for="label in label_options" :value="label.label_id">

                        {{label.name}}
                        
                    </option>

                    <option :value="null">None</option>

                </select>



                
            </span>

        </div>
        

        <div class="row">

            Attributes
            <span class="attribute-span">
                <div class="class-fields" v-for="class_fields in fields.attributes">
                    {{ class_fields.name }}
                    
                    <input v-model="attributes[class_fields.class_instance_id][field.name]" v-for="field in class_fields.fields" :type="field.object_type" :placeholder="field.name">
                </div>
            </span>
            
        </div>

        <div class="table">

            <table>

                <thead>
                    <tr>
                        <th>Origin</th>
                        <th>Part Number</th>
                        <th>Price</th>
                    </tr>
                </thead>


                <tbody>
                    <template v-for="(origin, index) in origins">

                        

                            <tr>

                                <td>
                                    <select v-model="origins[index]['origin_id']">

                                        <option v-for="option in origin_options" :value="option.origin_id">

                                            {{option.name}}
                                            
                                        </option>

                                    </select>
                                </td>
                                <td><input type="text" v-model="origins[index]['part_number']"></td>
                                <td><input type="text" v-model.number="origins[index]['price']"></td>
                                
                                <td><img class="remove" @click="remove_origin_row(index)" src="/images/remove.svg"></td>
                            </tr>


                    
                        
                    </template>
                </tbody>

                <tr class="add">
                    <img @click="add_origin_row" src="/images/add.svg">
                </tr>
            </table>    
        </div>
        
        <div class="row">
            Files

            <span>
                <FileUpload class="file" ref="img_uploader" text="Upload Image" accept="image/*" />
                <FileUpload class="file" ref="file_uploader" text="Upload Files" multiple accept="*" />
            </span>
        </div>

        
        



        <button class="button" @click="confirm">Confirm</button>



        
    </div>
</template>


<script setup lang="ts">
// need `class_instance_id` & `class_name`

import { get_all_classes, post_class, post_class_instance_id_get_class } from '@/api/class';
import { get_fields_from_class_instance, get_fields_from_class_instance_for_html, post_class_instance } from '@/api/class_instance';
import { pushAppError } from '@/error/error_state';
import { onBeforeMount, ref, useTemplateRef, type Ref } from 'vue';
import { clearActivePopup, opts, onSuccess } from './popup_state';
import { post_component, post_component_with_files } from '@/api/component';
import { get_all_manufacturers } from '@/api/manufacturer';
import { get_all_labels } from '@/api/label';
import FileUpload from '../components/FileUpload.vue';
import { get_all_origins } from '@/api/origin.ts';


    const class_: any = ref({})
    const fields: any = ref({})
    
    const manufacturer_options: Ref<Array<any>> = ref([])
    const label_options: Ref<Array<any>> = ref([])
    const origin_options: Ref<Array<any>> = ref([])

    const name = ref()
    const stock = ref()
    const manufacturer: Ref<any> = ref()
    const label: Ref<any> = ref()
    const origins: Ref<Array<any>> = ref([])
    
    const attributes: Ref<any> = ref({})

    const file_uploader: Ref<any> = useTemplateRef("file_uploader") 
    const img_uploader = useTemplateRef("img_uploader") 


    const emptyOrigin: Object = {
        origin_id: "",
        part_number: "",
        price: 0
    }

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

            // let manufacturer_out;
            // let label_out;

            let img_out: File | null = null;


            // if (manufacturer.value.length == 0) {
            //     manufacturer_out = null
            // } else {
            //     manufacturer_out = manufacturer.value.at(0)
            // }

            // if (label.value.length == 0) {
            //     label_out = null
            // } else {
            //     label_out = label.value.at(0)
            // }


            const temp = img_uploader.value?.files.at(0)

            if (temp != undefined) {
                img_out = temp
            } else {
                img_out = null
            }


            await post_component_with_files(
                opts.value.class_instance_id,
                name.value,
                stock.value,
                manufacturer.value,
                label.value,
                attributes.value,
                origins.value,
                file_uploader.value.files,
                img_out
            )

            if (onSuccess != null) {
                await onSuccess.value?.()
            }

            clearActivePopup()

        } catch(e: any) {
            pushAppError(e)
        }

    }


    function add_origin_row() {
        
        origins.value.push(structuredClone(emptyOrigin))
    }

    function remove_origin_row(index: number) {

        origins.value.splice(index, 1)

    }


    async function setup() {


        try {
            class_.value = await post_class_instance_id_get_class(opts.value.class_instance_id)
            fields.value = await get_fields_from_class_instance_for_html(opts.value.class_instance_id).then((res: any) => {
                

                for (let attr of res.attributes) {

                    attributes.value[attr.class_instance_id] = {}

                    for (let field of attr.fields) {
                        attributes.value[attr.class_instance_id][field.name] = null
                    }
                }



                return res


            })
            manufacturer_options.value = await get_all_manufacturers()
            label_options.value = await get_all_labels()
            origin_options.value = await get_all_origins()

        } catch (e: any) {
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

        grid-template-rows: 10px 30px 130px auto 100px 50px;
        row-gap: 20px;

        overflow-x: hidden;
        overflow-y: scroll;
        scrollbar-width: none;
    }


    input {
        width: 100%;
        height: 20px;
        margin-right: 5px;
        box-sizing: border-box;
        margin-bottom: 2px;
        
    }

    .core-input {
        height: 100%;
    }

    span {
        display: flex;
        flex-direction: row;
        height: 100%;
        box-sizing: border-box;
        
    }

    select {
        width: 100%;
        margin-right: 5px;
        box-sizing: border-box;
        //height: 100%;
    }

    .attribute-span {
        overflow-x: scroll;
    }

    .class-fields {
        background-color: import.$secondary;
        width: 150px;
        padding: 5px;
        border-radius: 5px;
        margin-right: 5px;
        height: 100%;
        box-sizing: border-box;

    }

    .file {
        margin-right: 5px;
        border-radius: 5px;
        background-color: import.$secondary;
        width: 100%;
        padding: 5px;
        
    }   

    .row {
        box-sizing: border-box;
    }

    .table {
        box-sizing: border-box;
    }


</style>