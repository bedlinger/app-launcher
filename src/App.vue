<template>
    <q-layout view="hHh lpR fFf">
        <AppHeader @update-open-shortcut="updateOpenShortcut" />
        <q-page-container>
            <q-page class="q-pa-md">
                <q-input ref="search" v-model="search" placeholder="Search" filled dense autofocus />
                <q-list bordered separator>
                    <AppItem v-for="(app, _index) in filteredApps" :key="app.path.toString()" :app="app"
                        @clearSearch="search = ''" />
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
import { isRegistered, register, unregister } from '@tauri-apps/api/globalShortcut'

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
            currentOpenShortcut: ''
        }
    },
    methods: {
        async getInstalledApps() {
            this.apps = await invoke('get_installed_apps')
        },
        async updateOpenShortcut(newShortcut: string) {
            if (this.currentOpenShortcut) {
                await unregister(this.currentOpenShortcut)
            }
            await register(newShortcut, async () => {
                if (await appWindow.isVisible()) {
                    await appWindow.hide();
                    this.search = ''
                    window.scrollTo(0, 0)
                } else {
                    await appWindow.show();
                    await appWindow.setFocus();
                    (this.$refs.search as HTMLInputElement).focus()
                }
            })
            this.currentOpenShortcut = newShortcut
        },
        async updateCloseShortcut() {
            if (!await isRegistered('Alt+F4')) {
                await register('Alt+F4', async () => {
                    await appWindow.close()
                })
            }
        }
    },
    computed: {
        filteredApps(): App[] {
            return this.apps.filter(app => app.name.toLowerCase().includes(this.search.toLowerCase()))
        }
    },
    mounted() {
        this.getInstalledApps()
        appWindow.center()
        this.updateCloseShortcut()
    }
}
</script>

<style>
::-webkit-scrollbar {
    display: none;
}
</style>