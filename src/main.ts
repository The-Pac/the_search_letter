import {createApp} from 'vue'
import App from './app/App.vue'
import './app/bootstrap.scss'
import {addIcons, OhVueIcon} from "oh-vue-icons";
import router from "./app/Router";
import {RiLoader5Line} from "oh-vue-icons/icons/ri";

const app = createApp(App)


//ICONS
addIcons(
    RiLoader5Line
)
app.component("v-icon", OhVueIcon);

//ROUTES
app.use(router)

export interface HTMLInputEvent extends Event {
    target: HTMLInputElement
}

//MOUNT
app.mount('#app');
