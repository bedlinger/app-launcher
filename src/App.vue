<template>
    <q-toolbar class="bg-grey-9 text-white">
        <q-toolbar-title>
            App-Launcher
        </q-toolbar-title>
        <q-btn flat round dense @click="showSettings = true">
            <q-icon name="more_vert" />
        </q-btn>
    </q-toolbar>
</template>

<script lang="ts">
import { appWindow } from '@tauri-apps/api/window'

export default {
    name: "App",
    data() {
        return {
            showSettings: false
        }
    },
    methods: {
    },
    mounted() {
        appWindow.center().catch(console.error)
    }
}
</script>