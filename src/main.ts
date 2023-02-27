import {createApp} from 'vue'
import App from './app/App.vue'
import './app/bootstrap.scss'
import {addIcons, OhVueIcon} from "oh-vue-icons";
import {CoArrowThickToBottom, CoArrowThickToTop, CoExitToApp, CoMediaPlay} from "oh-vue-icons/icons/co";
import {BiPeopleFill} from "oh-vue-icons/icons/bi";
import {FaRegularWindowMaximize, FaRegularWindowMinimize} from "oh-vue-icons/icons/fa";

import router from "./app/Router";

const app = createApp(App)


//ICONS
addIcons(
    CoMediaPlay,
    BiPeopleFill,
    CoExitToApp,
    FaRegularWindowMaximize,
    FaRegularWindowMinimize,
    CoArrowThickToBottom,
    CoArrowThickToTop
)
app.component("v-icon", OhVueIcon);

//ROUTES
app.use(router)

export interface HTMLInputEvent extends Event {
    target: HTMLInputElement
}

//MOUNT
app.mount('#app');
