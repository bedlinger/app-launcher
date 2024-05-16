<template>
    <q-dialog v-model="showSettings">
        <q-card>
            <q-toolbar>
                <q-toolbar-title><span class="text-weight-bold">Settings</span></q-toolbar-title>
                <q-btn flat round dense icon="close" @click="$emit('close')" />
            </q-toolbar>
            <q-card-section>
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Rerum repellendus sit voluptate voluptas
                eveniet porro. Rerum blanditiis perferendis totam, ea at omnis vel numquam exercitationem aut, natus
                minima, porro labore.
            </q-card-section>
        </q-card>
    </q-dialog>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
    name: 'SettingsModal',
    props: {
        showSettings: {
            type: Boolean,
            default: false,
            required: true
        }
    }
})
</script>