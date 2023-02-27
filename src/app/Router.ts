import {createRouter, createWebHistory, RouteRecordRaw} from 'vue-router';


export const home: RouteRecordRaw = {
    path: '/',
    name: 'Home',
    component: () => import("../pages/Home.vue")
}
export const error_page: RouteRecordRaw = {
    path: '/:pathMatch(.*)*',
    name: 'Error-Page',
    component: () => import("../pages/Error.vue")
}


const routes: Array<RouteRecordRaw> = [
    home,
    error_page
]

const router = createRouter({
    history: createWebHistory(),
    routes
});

export default router;
