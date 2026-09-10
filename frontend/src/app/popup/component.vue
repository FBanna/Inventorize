
<template>
    <div class="popup_template menu">

        Component

        {{ opts.component.name }}

        
    </div>
</template>


<script setup lang="ts">

import { get_all_classes, post_class } from '@/api/class';
import { post_class_instance } from '@/api/class_instance';
import { pushAppError } from '@/error/error_state';
import { ref, type Ref } from 'vue';
import { clearActivePopup, opts, onSuccess } from './popup_state';
import { post_label } from '@/api/label';




</script>



<style lang="scss" scoped>

    @use "@/style/import";

    .menu{

        width: 500px;
        height: 150px;

        display: grid;
        //grid-template-rows: 15px 20px 20px 50px;
        //row-gap: 10px;
    }




</style>