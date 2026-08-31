<template>

  <div>
    <label :for="label" class="button">{{ props.text }}</label>

    <input
      :id="label"
      type="file"
      :multiple="multiple"
      :accept="accept"
      @change="handleFileSelect"
      hidden
    />

    <ul v-if="files.length" class="file-list">
      <li v-for="(file, index) in files" :key="`${file.name}-${index}`">

        <img v-if="file.type.startsWith('image/')" :src="getPreviewUrl(file)" class="file-preview" />

        <span>{{ file.name }} ({{ (file.size / 1024).toFixed(1) }} KB)</span>
        <button @click="removeFile(index)">Remove</button>
      </li>
    </ul>

    

  </div>

</template>

<script setup lang="ts">
import { ref, useId, type Ref, } from 'vue'



const props = withDefaults(defineProps<{
  multiple?: boolean
  text: String
  accept?: string
}>(), {
  multiple: false,
  accept: 'image/*,application/pdf'
})

let label = useId()


const files: Ref<File[]> = ref([])
defineExpose({files})

function handleFileSelect(e: Event) {

    console.log(props.text)
  const input = e.target as HTMLInputElement
  const selectedFiles = Array.from(input?.files || [])

//   const validFiles = selectedFiles.filter(file => {
//     const sizeMB = file.size / (1024 * 1024)
//     if (sizeMB > props.maxSizeMB) {
//       errorMessage.value = `File "${file.name}" exceeds the ${props.maxSizeMB}MB size limit.`
//       return false
//     }
//     return true
//   })

  files.value = files.value.concat(selectedFiles)
}


function removeFile(index: number) {
  files.value.splice(index, 1)
}

function getPreviewUrl(file: File): string {
  return URL.createObjectURL(file)
}


</script>

<style lang="scss" scoped>

    @use "@/style/import";

</style>