<template>
    <q-dialog v-model="showSettings" transition-show="slide-up" transition-hide="slide-down"
        @escape-key="() => close()">
        <q-card style="width: 70%;">
            <q-toolbar>
                <q-toolbar-title><span class="text-weight-bold">Settings</span></q-toolbar-title>
                <q-btn flat round dense icon="close" @click="close()" />
            </q-toolbar>
            <q-separator />
            <q-card-section>
                <q-toggle label="Dark Mode" v-model="darkMode" class="q-pr-md" />
                <q-toggle label="Auto Start" v-model="autostart" />
                <p class="text-h6">Primary Color</p>
                <q-color v-model="color" no-header no-footer style="max-width: 250px;" />
            </q-card-section>
        </q-card>
    </q-dialog>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { Dark } from 'quasar'
import { enable, disable } from "tauri-plugin-autostart-api"

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
            autostart: true,
            color: '#3758ef'
        }
    },
    methods: {
        close() {
            this.$emit('close')
            this.saveSettings()
        },
        saveSettings() {
            localStorage.setItem('settings', JSON.stringify({ darkMode: this.darkMode, autostart: this.autostart, color: this.color }))
        },
        loadSettings() {
            const settings = JSON.parse(localStorage.getItem('settings') || '{}')
            this.darkMode = settings.darkMode !== undefined ? settings.darkMode : false
            this.autostart = settings.autostart !== undefined ? settings.autostart : true
            this.color = settings.color || '#3758ef'
        }
    },
    watch: {
        darkMode(value: boolean) {
            Dark.set(value)
        },
        async autostart(value: boolean) {
            if (value) {
                await enable()
            } else {
                await disable()
            }
        },
        color(value: string) {
            document.body.style.setProperty('--q-primary', value)
        }
    },
    mounted() {
        this.loadSettings()
    }
})
</script>