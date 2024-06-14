<template>
    <q-item clickable @click="launchApp(app)">
        <q-item-section avatar>
            <q-icon name="apps" />
        </q-item-section>
        <q-item-section>
            {{ app.name }}
        </q-item-section>
        <q-btn label="Open location" no-caps ripple flat dense icon="launch" tabindex="-1" @click.stop="openLocation($event, app)">
            <q-tooltip>{{ app.path }}</q-tooltip>
        </q-btn>
    </q-item>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import App from '../types/App'
import { appWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api'
import { type } from '@tauri-apps/api/os'

export default defineComponent({
    name: 'AppItem',
    props: {
        app: {
            type: Object as () => App,
            required: true
        }
    },
    methods: {
        async launchApp(app: App) {
            const os = await type()
            if (os === 'Windows_NT') {
                invoke('launch_app_win', { app })
                    .catch(console.error)
                    .then(_onfulfilled => {
                        appWindow.hide()
                        this.$emit('clearSearch')
                    })
            }
            if (os === 'Linux') {
                invoke('launch_app_linux', { app })
                    .catch(console.error)
                    .then(_onfulfilled => {
                        appWindow.hide()
                        this.$emit('clearSearch')
                    })
            }
        },
        async openLocation(event: Event, app: App) {
            event.stopPropagation();
            const os = await type()
            if (os === 'Windows_NT') {
                invoke('open_file_location_win', { app })
            }
            if (os === 'Linux') {
                // TODO: Implement open_file_location_linux
                //invoke('open_file_location_linux', { app })
            }
        }
    }
})
</script>