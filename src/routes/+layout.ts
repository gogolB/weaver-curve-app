// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://beta.tauri.app/start/frontend/sveltekit/ for more info
import "tailwindcss/tailwind.css";
export const prerender = true;
export const ssr = false;
