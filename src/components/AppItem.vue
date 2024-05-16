<template>
    <q-item clickable @click="launchApp(app)">
        <q-item-section avatar>
            <q-icon name="apps" />
        </q-item-section>
        <q-item-section>
            {{ app.name }}
        </q-item-section>
        <q-btn label="Open location" no-caps ripple flat dense icon="launch" @click.stop="openLocation($event, app)" />
    </q-item>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import App from '../types/App'
import { invoke } from '@tauri-apps/api'

export default defineComponent({
    name: 'AppItem',
    props: {
        app: {
            type: Object as () => App,
            required: true
        }
    },
    methods: {
        launchApp(app: App) {
            invoke('launch_app', { app })
                .catch(console.error)
                .then(_onfullfilled => {
                    // appWindow.close()
                })
        },
        openLocation(event: Event, app: App) {
            event.stopPropagation();
            invoke('open_location', { app })
                .catch(console.error)
        }
    }
})
</script>