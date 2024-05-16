<template>
    <q-dialog v-model="showSettings" transition-show="slide-up" transition-hide="slide-down" @escape-key="() => $emit('close')">
        <q-card style="width: 40%; height: 40%;">
            <q-toolbar>
                <q-toolbar-title><span class="text-weight-bold">Settings</span></q-toolbar-title>
                <q-btn flat round dense icon="close" @click="$emit('close')" />
            </q-toolbar>
            <q-separator />
            <q-card-section>
                <q-toggle label="Dark Mode?" v-model="darkMode" />
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
            darkMode: Dark.isActive
        }
    },
    watch: {
        darkMode(_value: boolean) {
            Dark.toggle()
        }
    }
})
</script>