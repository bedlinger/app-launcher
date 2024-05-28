<template>
    <q-dialog v-model="showSettings" transition-show="slide-up" transition-hide="slide-down"
        @escape-key="() => close">
        <q-card style="width: 70%;">
            <q-toolbar>
                <q-toolbar-title><span class="text-weight-bold">Settings</span></q-toolbar-title>
                <q-btn flat round dense icon="close" @click="close" />
            </q-toolbar>
            <q-separator />
            <q-card-section>
                <div class="row">
                    <q-toggle label="Dark Mode" v-model="darkMode" class="col" />
                    <q-toggle label="Auto Start" v-model="autostart" class="col" />
                    <q-input class="col" label="Shortcut" v-model="shortcut" outlined dense
                        @keydown="updateOpenShortcut" />
                </div>
                <div class=" column items-center justify-center q-pt-sm">
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
            keys: [] as string[],
            color: '#3758ef'
        }
    },
    methods: {
        close() {
            this.$emit('close')
            this.saveSettings()
        },
        updateOpenShortcut(event: KeyboardEvent) {
            this.keys = []
            if (event.key !== 'Meta' && event.key !== 'Control' && event.key !== 'Shift' && event.key !== 'Alt' && event.key !== 'CapsLock' && event.key !== 'Tab' && event.key !== 'Escape' && event.key !== 'Enter' && event.key !== 'Backspace' && event.key !== 'Delete' && event.key !== 'ArrowUp' && event.key !== 'ArrowDown' && event.key !== 'ArrowLeft' && event.key !== 'ArrowRight' && event.key !== 'Home' && event.key !== 'End' && event.key !== 'PageUp' && event.key !== 'PageDown' && event.key !== 'Insert' && event.key !== 'PrintScreen' && event.key !== 'ScrollLock' && event.key !== 'Pause') {
                this.keys.push(event.key.toUpperCase())
            }
            if (event.ctrlKey) {
                this.keys.push('Ctrl')
            }
            if (event.metaKey) {
                this.keys.push('Meta')
            }
            if (event.shiftKey) {
                this.keys.push('Shift')
            }
            if (event.altKey) {
                this.keys.push('Alt')
            } 
            this.shortcut = this.keys.reverse().join('+')
            this.$emit('update-open-shortcut', this.shortcut)
        },
        saveSettings() {
            localStorage.setItem('settings', JSON.stringify({
                darkMode: this.darkMode,
                autostart: this.autostart,
                color: this.color,
                shortcut: this.shortcut
            }))
        },
        loadSettings() {
            const settings = JSON.parse(localStorage.getItem('settings') || '{}')
            this.darkMode = settings.darkMode !== undefined ? settings.darkMode : false
            this.autostart = settings.autostart !== undefined ? settings.autostart : true
            this.color = settings.color || '#3758ef'
            this.shortcut = settings.shortcut || 'Alt+S'
            this.$emit('update-open-shortcut', this.shortcut)
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