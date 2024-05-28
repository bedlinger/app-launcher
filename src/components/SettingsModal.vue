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
                <div class="row">
                    <q-toggle label="Dark Mode" v-model="darkMode" class="col" />
                    <q-toggle label="Auto Start" v-model="autostart" class="col" />
                    <q-input label="Shortcut" v-model="shortcut" outlined dense class="col" />
                </div>
                <div class="column items-center justify-center q-pt-sm">
                    <p class="text-h6">Primary Color</p>
                    <q-color v-model="color" no-header no-footer style="width: 270px;" />
                </div>
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
            shortcut: 'Alt+S',
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