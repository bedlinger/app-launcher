<template>
    <q-layout view="hHh lpR fFf">
        <AppHeader />
        <q-page-container>
            <q-page class="q-pa-md bg-white">
                <q-input v-model="search" placeholder="Search" filled dense autofocus debounce="200" />
                <q-list bordered separator>
                    <AppItem v-for="app in filteredApps" :key="app.path.toString()" :app="app" />
                </q-list>
            </q-page>
        </q-page-container>
    </q-layout>
</template>

<script lang="ts">
import App from './types/App'
import AppHeader from './components/AppHeader.vue'
import AppItem from './components/AppItem.vue'
import { appWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api'

export default {
    name: "App",
    components: {
        AppHeader,
        AppItem
    },
    data() {
        return {
            apps: [] as App[],
            search: ''
        }
    },
    methods: {
        async getInstalledApps() {
            this.apps = await invoke('get_installed_apps')
        }
    },
    computed: {
        filteredApps() {
            return this.apps.filter(app => app.name.toLowerCase().includes(this.search.toLowerCase()))
        }
    },
    mounted() {
        this.getInstalledApps()
        appWindow.center()
    }
}
</script>