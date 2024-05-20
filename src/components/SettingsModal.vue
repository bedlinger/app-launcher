<template>
    <q-dialog v-model="showSettings" transition-show="slide-up" transition-hide="slide-down"
        @escape-key="() => $emit('close')">
        <q-card style="width: 70%;">
            <q-toolbar>
                <q-toolbar-title><span class="text-weight-bold">Settings</span></q-toolbar-title>
                <q-btn flat round dense icon="close" @click="$emit('close')" />
            </q-toolbar>
            <q-separator />
            <q-card-section>
                <q-toggle label="Dark Mode?" v-model="darkMode" />
                <p class="text-h6">Primary Color</p>
                <q-color v-model="color" no-header no-footer style="max-width: 250px;" />
            </q-card-section>
        </q-card>
    </q-dialog>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { Dark } from 'quasar'

export default defineComponent({
    name: 'SettingsModal',
    props: {
        showSettings: {
            type: Boolean,
            default: false,
            required: true
        }
    },
    data() {
        return {
            darkMode: false,
            color: '#3758ef'
        }
    },
    watch: {
        darkMode(value: boolean) {
            Dark.set(value)
        },
        color(value: string) {
            document.body.style.setProperty('--q-primary', value)
        }
    }
})
</script>