import { createApp } from 'vue';

import i18next from 'i18next';
import I18NextVue from 'i18next-vue';
import LanguageDetector from 'i18next-browser-languagedetector';

import App from './app.vue';
import locales from './locales.json';

const app = createApp(App);

const resources = {};
Object.keys(locales).forEach((lng) => {
    resources[lng] = {
        translation: locales[lng]
    };
});

i18next.use(LanguageDetector).init({
    resources
});

app.use(I18NextVue, {
    i18next
});

app.mount('body');
