<template>
    <q-layout view="hHh lpR fFf">
        <AppHeader />
        <q-page-container>
            <q-page :class="`q-pa-md ${darkMode ? 'bg-dark text-light' : 'text-dark'}`">
                <q-input v-model="search" placeholder="Search" filled dense autofocus debounce="200" />
                <q-list bordered separator>
                    <AppItem v-for="app in filteredApps" :key="app.path.toString()" :app="app" />
                </q-list>
            </q-page>
        </q-page-container>
    </q-layout>
</template>

<script lang="ts">
import { Dark } from 'quasar'
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
            search: '',
            darkMode: Dark.isActive
        }
    },
    methods: {
        async getInstalledApps() {
            this.apps = await invoke('get_installed_apps')
        },
        saveSetTheme() {
            localStorage.setItem('darkMode', this.darkMode.toString())
        },
        async loadSetTheme() {
            const darkMode = localStorage.getItem('darkMode')
            if (darkMode) {
                this.darkMode = darkMode === 'true'
                Dark.set(this.darkMode)
            }
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
        this.loadSetTheme()
    }, 
    unmounted() {
        this.saveSetTheme()
    }
}
</script>